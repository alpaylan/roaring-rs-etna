// ETNA workload runner for roaring-rs.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: IterMatchesModel | AdvanceToMatchesModel | AdvanceBackToMatchesModel
//             | IterNthMatchesModel | RangeCardinalityMatchesModel | All
//
// Every invocation prints exactly one JSON line to stdout and exits 0
// (except argv parsing which exits 2).

use crabcheck::quickcheck as crabcheck_qc;
use crabcheck::quickcheck::Arbitrary as CcArbitrary;
use hegel::{generators as hgen, HealthCheck, Hegel, Settings as HegelSettings, TestCase};
use proptest::prelude::*;
use proptest::test_runner::{Config as ProptestConfig, TestCaseError, TestError, TestRunner};
use quickcheck::{Arbitrary as QcArbitrary, Gen, QuickCheck, ResultStatus, TestResult};
use rand::Rng;
use roaring::etna::{
    property_advance_back_to_matches_model, property_advance_to_matches_model,
    property_iter_matches_model, property_iter_nth_matches_model,
    property_range_cardinality_matches_model, PropertyResult,
};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "IterMatchesModel",
    "AdvanceToMatchesModel",
    "AdvanceBackToMatchesModel",
    "IterNthMatchesModel",
    "RangeCardinalityMatchesModel",
];

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if let Err(e) = r {
            return (Err(e), total);
        }
    }
    (Ok(()), total)
}

// ---------- Canonical witness builders ----------

fn single_full_container_spec() -> (Vec<u32>, Vec<(u32, u16)>) {
    (Vec::new(), vec![(0, u16::MAX)])
}

fn two_full_containers_spec() -> (Vec<u32>, Vec<(u32, u16)>) {
    (Vec::new(), vec![(0, u16::MAX), (65_536, u16::MAX)])
}

fn compressed_run_spec() -> (Vec<u32>, Vec<(u32, u16)>) {
    let mut r: Vec<(u32, u16)> = Vec::new();
    let mut x: u32 = 500;
    while x < 0x35B00 {
        r.push((x, 2));
        x += 7;
    }
    (Vec::new(), r)
}

fn bitmap_store_spec() -> (Vec<u32>, Vec<(u32, u16)>) {
    let pts: Vec<u32> = (0u32..=16_384).filter(|&x| x % 3 == 0).collect();
    (pts, Vec::new())
}

fn check_iter_matches_model() -> Result<(), String> {
    let (vs, rs) = two_full_containers_spec();
    to_err(property_iter_matches_model(vs, rs))
}

fn check_advance_to_matches_model() -> Result<(), String> {
    let (vs, rs) = compressed_run_spec();
    to_err(property_advance_to_matches_model(vs, rs, u32::MAX, 0x35B01))?;
    let (vs, rs) = bitmap_store_spec();
    to_err(property_advance_to_matches_model(vs, rs, 100, 300))
}

fn check_advance_back_to_matches_model() -> Result<(), String> {
    let (vs, rs) = compressed_run_spec();
    to_err(property_advance_back_to_matches_model(vs, rs, 0, 499))?;
    let (vs, rs) = bitmap_store_spec();
    to_err(property_advance_back_to_matches_model(vs, rs, 300, 100))
}

fn check_iter_nth_matches_model() -> Result<(), String> {
    let (vs, rs) = single_full_container_spec();
    to_err(property_iter_nth_matches_model(vs, rs, 65_535, 100_000))
}

fn check_range_cardinality_matches_model() -> Result<(), String> {
    let (vs, rs) = single_full_container_spec();
    to_err(property_range_cardinality_matches_model(vs, rs, 64, 6400))
}

// ---------- etna (deterministic witness replay) ----------

fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "IterMatchesModel" => check_iter_matches_model(),
        "AdvanceToMatchesModel" => check_advance_to_matches_model(),
        "AdvanceBackToMatchesModel" => check_advance_back_to_matches_model(),
        "IterNthMatchesModel" => check_iter_nth_matches_model(),
        "RangeCardinalityMatchesModel" => check_range_cardinality_matches_model(),
        _ => {
            return (
                Err(format!("Unknown property for etna: {property}")),
                Metrics::default(),
            );
        }
    };
    (
        result,
        Metrics {
            inputs: 1,
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------- shared Arbitrary-biased generators (qc + cc) ----------
//
// Newtype wrappers around the raw `(Vec<u32>, Vec<(u32, u16)>)` spec. Each
// wrapper carries two Arbitrary impls:
//   * `quickcheck::Arbitrary` — used by the quickcheck fork
//   * `crabcheck::quickcheck::Arbitrary<R>` — used by crabcheck
// Both mirror the proptest strategies: values 0..200_000, lengths bounded,
// range-start weighted 40/40/20 on {0, 65_536, uniform 0..200_000}, range-len
// weighted 50/25/25 on {u16::MAX, 0..=100, 100..=4096}. Debug delegates to
// the inner type so the counterexample reads as a plain Vec/primitive across
// frameworks.
#[derive(Clone)]
struct Values(Vec<u32>);

#[derive(Clone)]
struct Ranges(Vec<(u32, u16)>);

#[derive(Clone, Copy)]
struct Pos200K(u32);

impl fmt::Debug for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for Pos200K {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Pos200K {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl QcArbitrary for Values {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.random_range(0usize..=16_384);
        let vs = (0..len).map(|_| g.random_range(0u32..200_000)).collect();
        Values(vs)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Values))
    }
}

impl QcArbitrary for Ranges {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.random_range(0usize..=4);
        let rs = (0..len)
            .map(|_| {
                let start = match g.random_range(0u8..5) {
                    0 | 1 => 0u32,
                    2 | 3 => 65_536u32,
                    _ => g.random_range(0u32..200_000),
                };
                let len = match g.random_range(0u8..4) {
                    0 | 1 => u16::MAX,
                    2 => g.random_range(0u16..=100),
                    _ => g.random_range(100u16..=4096),
                };
                (start, len)
            })
            .collect();
        Ranges(rs)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Ranges))
    }
}

impl QcArbitrary for Pos200K {
    fn arbitrary(g: &mut Gen) -> Self {
        Pos200K(g.random_range(0u32..200_000))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Pos200K))
    }
}

impl<R: Rng> CcArbitrary<R> for Values {
    fn generate(rng: &mut R, _n: usize) -> Self {
        let len = rng.random_range(0usize..=16_384);
        let vs = (0..len).map(|_| rng.random_range(0u32..200_000)).collect();
        Values(vs)
    }
}

impl<R: Rng> CcArbitrary<R> for Ranges {
    fn generate(rng: &mut R, _n: usize) -> Self {
        let len = rng.random_range(0usize..=4);
        let rs = (0..len)
            .map(|_| {
                let start = match rng.random_range(0u8..5) {
                    0 | 1 => 0u32,
                    2 | 3 => 65_536u32,
                    _ => rng.random_range(0u32..200_000),
                };
                let len = match rng.random_range(0u8..4) {
                    0 | 1 => u16::MAX,
                    2 => rng.random_range(0u16..=100),
                    _ => rng.random_range(100u16..=4096),
                };
                (start, len)
            })
            .collect();
        Ranges(rs)
    }
}

impl<R: Rng> CcArbitrary<R> for Pos200K {
    fn generate(rng: &mut R, _n: usize) -> Self {
        Pos200K(rng.random_range(0u32..200_000))
    }
}

// ---------- proptest ----------

// Proptest strategies biased to frequently hit bug-triggering patterns:
//   - lengths include u16::MAX (full container) as an explicit edge value,
//     so a24ff69 / c41bab3 / 3116bcc / 136b8f1 inputs occur naturally;
//   - starts cluster at 0 and at container boundaries (multiples of 65_536)
//     so full runs align to whole containers;
//   - `values_strategy` produces up to 16_384 scalars so a dense BitmapStore
//     container is reachable (needed for eaccd09).
fn values_strategy() -> BoxedStrategy<Vec<u32>> {
    prop::collection::vec(0u32..200_000u32, 0..=16_384).boxed()
}

fn range_start() -> BoxedStrategy<u32> {
    prop_oneof![
        2 => Just(0u32),
        2 => Just(65_536u32),
        1 => 0u32..200_000u32,
    ]
    .boxed()
}

fn range_len() -> BoxedStrategy<u16> {
    prop_oneof![
        2 => Just(u16::MAX),
        1 => 0u16..=100u16,
        1 => 100u16..=4096u16,
    ]
    .boxed()
}

fn ranges_strategy() -> BoxedStrategy<Vec<(u32, u16)>> {
    prop::collection::vec((range_start(), range_len()), 0..=4).boxed()
}

// For IterNthMatchesModel: the spec is fixed to a single full run container.
// The framework varies back_target and n. This keeps the property's
// `ranges.len() <= 1` precondition satisfied (never discards) and provokes
// the 318366d variant whenever `n > u16::MAX`.
fn run_proptest_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let cfg = ProptestConfig {
        cases: 40_000_000,
        max_shrink_iters: 32,
        ..ProptestConfig::default()
    };
    let mut runner = TestRunner::new(cfg);
    let c = counter.clone();
    let result: Result<(), String> = match property {
        "IterMatchesModel" => runner
            .run(&(values_strategy(), ranges_strategy()), move |(vs, rs)| {
                c.fetch_add(1, Ordering::Relaxed);
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_iter_matches_model(vs, rs)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!("({:?} {:?})", vs_cex, rs_cex))),
                }
            })
            .map_err(|e| match e { TestError::Fail(reason, _) => reason.to_string(), other => other.to_string() }),
        "AdvanceToMatchesModel" => runner
            .run(
                &(
                    values_strategy(),
                    ranges_strategy(),
                    0u32..200_000u32,
                    0u32..200_000u32,
                ),
                move |(vs, rs, back, target)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let vs_cex = vs.clone();
                    let rs_cex = rs.clone();
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        property_advance_to_matches_model(vs, rs, back, target)
                    ));
                    match outcome {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!("({:?} {:?} {} {})", vs_cex, rs_cex, back, target))),
                    }
                },
            )
            .map_err(|e| match e { TestError::Fail(reason, _) => reason.to_string(), other => other.to_string() }),
        "AdvanceBackToMatchesModel" => runner
            .run(
                &(
                    values_strategy(),
                    ranges_strategy(),
                    0u32..200_000u32,
                    0u32..200_000u32,
                ),
                move |(vs, rs, forward, target)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let vs_cex = vs.clone();
                    let rs_cex = rs.clone();
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        property_advance_back_to_matches_model(vs, rs, forward, target)
                    ));
                    match outcome {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!("({:?} {:?} {} {})", vs_cex, rs_cex, forward, target))),
                    }
                },
            )
            .map_err(|e| match e { TestError::Fail(reason, _) => reason.to_string(), other => other.to_string() }),
        "IterNthMatchesModel" => runner
            .run(
                // back_target is biased toward u16::MAX because the
                // 318366d trigger requires `advance_back_to(>=u16::MAX)`
                // to leave the container's backward_offset at 0. n is
                // always > u16::MAX to probe the removed guard.
                &(
                    prop_oneof![
                        3 => Just(u16::MAX as u32),
                        1 => 0u32..=65_535u32,
                    ],
                    65_536u32..200_000u32,
                ),
                move |(back, n)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let (vs, rs) = single_full_container_spec();
                    let vs_cex = vs.clone();
                    let rs_cex = rs.clone();
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        property_iter_nth_matches_model(vs, rs, back, n)
                    ));
                    match outcome {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!("({:?} {:?} {} {})", vs_cex, rs_cex, back, n))),
                    }
                },
            )
            .map_err(|e| match e { TestError::Fail(reason, _) => reason.to_string(), other => other.to_string() }),
        "RangeCardinalityMatchesModel" => runner
            .run(
                &(
                    values_strategy(),
                    ranges_strategy(),
                    0u32..200_000u32,
                    0u32..200_000u32,
                ),
                move |(vs, rs, start, end)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let (lo, hi) = if start <= end { (start, end) } else { (end, start) };
                    let vs_cex = vs.clone();
                    let rs_cex = rs.clone();
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        property_range_cardinality_matches_model(vs, rs, lo, hi)
                    ));
                    match outcome {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!("({:?} {:?} {} {})", vs_cex, rs_cex, lo, hi))),
                    }
                },
            )
            .map_err(|e| match e { TestError::Fail(reason, _) => reason.to_string(), other => other.to_string() }),
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ---------- quickcheck (forked crate with `etna` feature) ----------
//
// Arbitrary-driven: property args are the `Values` / `Ranges` newtypes defined
// above, which bias generation to mirror proptest's strategies. IterNth uses a
// fixed single-full-container spec and only varies back/n.

static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn qc_iter_matches_model(Values(vs): Values, Ranges(rs): Ranges) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_iter_matches_model(vs, rs) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_advance_to_matches_model(
    Values(vs): Values,
    Ranges(rs): Ranges,
    Pos200K(back): Pos200K,
    Pos200K(target): Pos200K,
) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_advance_to_matches_model(vs, rs, back, target) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_advance_back_to_matches_model(
    Values(vs): Values,
    Ranges(rs): Ranges,
    Pos200K(forward): Pos200K,
    Pos200K(target): Pos200K,
) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_advance_back_to_matches_model(vs, rs, forward, target) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

// IterNthMatchesModel: fixed single-full-container spec, vary back_target (u16)
// and n_extra (u16). n = 65_536 + n_extra guarantees n > u16::MAX and keeps the
// property's ranges.len() <= 1 precondition satisfied.
fn qc_iter_nth_matches_model(back_seed: u16, n_extra: u16) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (vs, rs) = single_full_container_spec();
    let back = back_seed as u32;
    let n = 65_536u32 + n_extra as u32;
    match property_iter_nth_matches_model(vs, rs, back, n) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_range_cardinality_matches_model(
    Values(vs): Values,
    Ranges(rs): Ranges,
    Pos200K(s): Pos200K,
    Pos200K(e): Pos200K,
) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
    match property_range_cardinality_matches_model(vs, rs, lo, hi) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let mut qc = QuickCheck::new()
        .tests(40_000_000)
        .max_tests(80_000_000)
        .max_time(Duration::from_secs(86_400));
    let result = match property {
        "IterMatchesModel" => {
            qc.quicktest(qc_iter_matches_model as fn(Values, Ranges) -> TestResult)
        }
        "AdvanceToMatchesModel" => qc.quicktest(
            qc_advance_to_matches_model as fn(Values, Ranges, Pos200K, Pos200K) -> TestResult,
        ),
        "AdvanceBackToMatchesModel" => qc.quicktest(
            qc_advance_back_to_matches_model as fn(Values, Ranges, Pos200K, Pos200K) -> TestResult,
        ),
        "IterNthMatchesModel" => {
            qc.quicktest(qc_iter_nth_matches_model as fn(u16, u16) -> TestResult)
        }
        "RangeCardinalityMatchesModel" => qc.quicktest(
            qc_range_cardinality_matches_model as fn(Values, Ranges, Pos200K, Pos200K) -> TestResult,
        ),
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!(
            "({})",
            arguments.join(" ")
        )),
        ResultStatus::Aborted { err } => Err(format!("quickcheck aborted: {err:?}")),
        ResultStatus::TimedOut => Err("quickcheck timed out".to_string()),
        ResultStatus::GaveUp => Err(format!(
            "quickcheck gave up after {} tests",
            result.n_tests_passed
        )),
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- crabcheck ----------
//
// Arbitrary-driven via the shared `Values`/`Ranges`/`Pos200K` wrappers defined
// above, which mirror proptest's biased strategies. IterNth uses a fixed
// single-full-container spec and only varies back/n (u16-sized inputs).

static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cc_iter_matches_model((Values(vs), Ranges(rs)): (Values, Ranges)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_iter_matches_model(vs, rs) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_advance_to_matches_model(
    (Values(vs), Ranges(rs), Pos200K(back), Pos200K(target)): (Values, Ranges, Pos200K, Pos200K),
) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_advance_to_matches_model(vs, rs, back, target) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_advance_back_to_matches_model(
    (Values(vs), Ranges(rs), Pos200K(forward), Pos200K(target)): (Values, Ranges, Pos200K, Pos200K),
) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_advance_back_to_matches_model(vs, rs, forward, target) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_iter_nth_matches_model((back_seed, n_extra): (usize, usize)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (vs, rs) = single_full_container_spec();
    let back = (back_seed & 0xFFFF) as u32;
    let n = 65_536u32 + (n_extra & 0xFFFF) as u32;
    match property_iter_nth_matches_model(vs, rs, back, n) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_range_cardinality_matches_model(
    (Values(vs), Ranges(rs), Pos200K(s), Pos200K(e)): (Values, Ranges, Pos200K, Pos200K),
) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
    match property_range_cardinality_matches_model(vs, rs, lo, hi) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let cc_config = crabcheck_qc::Config { tests: 40_000_000 };
    let result = match property {
        "IterMatchesModel" => crabcheck_qc::quickcheck_with_config(cc_config, cc_iter_matches_model),
        "AdvanceToMatchesModel" => crabcheck_qc::quickcheck_with_config(cc_config, cc_advance_to_matches_model),
        "AdvanceBackToMatchesModel" => crabcheck_qc::quickcheck_with_config(cc_config, cc_advance_back_to_matches_model),
        "IterNthMatchesModel" => crabcheck_qc::quickcheck_with_config(cc_config, cc_iter_nth_matches_model),
        "RangeCardinalityMatchesModel" => {
            crabcheck_qc::quickcheck_with_config(cc_config, cc_range_cardinality_matches_model)
        }
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        crabcheck_qc::ResultStatus::Finished => Ok(()),
        crabcheck_qc::ResultStatus::Failed { arguments } => {
            Err(format!("({})", arguments.join(" ")))
        },
        crabcheck_qc::ResultStatus::TimedOut => Err("crabcheck timed out".to_string()),
        crabcheck_qc::ResultStatus::GaveUp => Err(format!(
            "crabcheck gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        crabcheck_qc::ResultStatus::Aborted { error } => {
            Err(format!("crabcheck aborted: {error}"))
        }
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- hegel ----------

static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> HegelSettings {
    HegelSettings::new()
        .test_cases(40_000_000)
        .suppress_health_check(HealthCheck::all())
}

// Hegel draw helpers — mirror the proptest strategies (see
// `values_strategy`, `range_start`, `range_len`, `ranges_strategy`).
// Weights are simulated by drawing a tag integer and matching: hegel's
// `one_of!` is uniform, so the tag-match pattern is the standard way.

fn hg_draw_pos(tc: &TestCase) -> u32 {
    tc.draw(
        hgen::integers::<u32>()
            .min_value(0)
            .max_value(199_999),
    )
}

fn hg_draw_range_start(tc: &TestCase) -> u32 {
    // proptest weights: 2/2/1 on {0, 65_536, uniform 0..200_000}.
    let tag = tc.draw(hgen::integers::<u8>().min_value(0).max_value(4));
    match tag {
        0 | 1 => 0u32,
        2 | 3 => 65_536u32,
        _ => hg_draw_pos(tc),
    }
}

fn hg_draw_range_len(tc: &TestCase) -> u16 {
    // proptest weights: 2/1/1 on {u16::MAX, 0..=100, 100..=4096}.
    let tag = tc.draw(hgen::integers::<u8>().min_value(0).max_value(3));
    match tag {
        0 | 1 => u16::MAX,
        2 => tc.draw(hgen::integers::<u16>().min_value(0).max_value(100)),
        _ => tc.draw(hgen::integers::<u16>().min_value(100).max_value(4096)),
    }
}

fn hg_draw_spec(tc: &TestCase) -> (Vec<u32>, Vec<(u32, u16)>) {
    let vlen = tc.draw(
        hgen::integers::<usize>()
            .min_value(0)
            .max_value(16_384),
    );
    let values: Vec<u32> = (0..vlen).map(|_| hg_draw_pos(tc)).collect();
    let rlen = tc.draw(hgen::integers::<usize>().min_value(0).max_value(4));
    let ranges: Vec<(u32, u16)> = (0..rlen)
        .map(|_| (hg_draw_range_start(tc), hg_draw_range_len(tc)))
        .collect();
    (values, ranges)
}

fn run_hegel_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match property {
        "IterMatchesModel" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (vs, rs) = hg_draw_spec(&tc);
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_iter_matches_model(vs, rs)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?} {:?})", vs_cex, rs_cex),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "AdvanceToMatchesModel" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (vs, rs) = hg_draw_spec(&tc);
                let b = hg_draw_pos(&tc);
                let t = hg_draw_pos(&tc);
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_advance_to_matches_model(vs, rs, b, t)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?} {:?} {} {})", vs_cex, rs_cex, b, t),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "AdvanceBackToMatchesModel" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (vs, rs) = hg_draw_spec(&tc);
                let f = hg_draw_pos(&tc);
                let t = hg_draw_pos(&tc);
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_advance_back_to_matches_model(vs, rs, f, t)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?} {:?} {} {})", vs_cex, rs_cex, f, t),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "IterNthMatchesModel" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (vs, rs) = single_full_container_spec();
                let back = tc.draw(hgen::integers::<u16>()) as u32;
                let n = 65_536u32 + tc.draw(hgen::integers::<u16>()) as u32;
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_iter_nth_matches_model(vs, rs, back, n)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?} {:?} {} {})", vs_cex, rs_cex, back, n),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "RangeCardinalityMatchesModel" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (vs, rs) = hg_draw_spec(&tc);
                let s = hg_draw_pos(&tc);
                let e = hg_draw_pos(&tc);
                let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
                let vs_cex = vs.clone();
                let rs_cex = rs.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                    property_range_cardinality_matches_model(vs, rs, lo, hi)
                ));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?} {:?} {} {})", vs_cex, rs_cex, lo, hi),
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{}", property),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(msg.strip_prefix("Property test failed: ").unwrap_or(&msg).to_string())
        }
    };
    (status, metrics)
}

// ---------- dispatch ----------

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (
            Err(format!("Unknown tool: {tool}")),
            Metrics::default(),
        ),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: IterMatchesModel | AdvanceToMatchesModel | AdvanceBackToMatchesModel | IterNthMatchesModel | RangeCardinalityMatchesModel | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(tool, property, "aborted", Metrics::default(), None, Some(&msg));
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(e) => emit_json(tool, property, "failed", metrics, Some(&e), None),
    }
}
