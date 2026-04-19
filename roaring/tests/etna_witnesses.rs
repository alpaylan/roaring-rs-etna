// Deterministic witness tests for ETNA variants.
//
// Each `witness_<name>_case_<tag>` passes on the base commit and fails under
// the corresponding `etna/<variant>` branch (where the historical bug has
// been re-injected via a patch). Witnesses call `property_<name>` directly
// with frozen inputs; they do not touch framework machinery.

use roaring::etna::{
    property_advance_back_to_matches_model, property_advance_to_matches_model,
    property_iter_matches_model, property_iter_nth_matches_model,
    property_range_cardinality_matches_model, PropertyResult,
};

fn expect_pass(r: PropertyResult, what: &str) {
    match r {
        PropertyResult::Pass => {}
        PropertyResult::Fail(m) => panic!("{}: property failed: {}", what, m),
        PropertyResult::Discard => panic!("{}: unexpected discard", what),
    }
}

// Two back-to-back full containers. Exercises forward/backward RunIter
// offset overflow at the boundary between two full runs: iterating past
// the last element of container 0 must consume that interval and reset
// `forward_offset` to 0 before yielding container 1's first element.
// Symmetric condition holds for `rev()`.
fn two_full_containers() -> (Vec<u32>, Vec<(u32, u16)>) {
    (Vec::new(), vec![(0, u16::MAX), (65_536, u16::MAX)])
}

// Many small runs packed across several containers: exercises RunIter's
// advance_to / advance_back_to past-end and past-start branches.
fn compressed_run_ranges() -> (Vec<u32>, Vec<(u32, u16)>) {
    let mut r: Vec<(u32, u16)> = Vec::new();
    let mut x: u32 = 500;
    while x < 0x35B00 {
        r.push((x, 2));
        x += 7;
    }
    (Vec::new(), r)
}

// A dense BitmapStore container: every 3rd value in 0..=16_384 yields ~5500
// points, above ArrayStore's 4096 cap but non-consecutive so the container
// stays as BitmapStore (exercises `BitmapIter`).
fn bitmap_store_spec() -> (Vec<u32>, Vec<(u32, u16)>) {
    let pts: Vec<u32> = (0u32..=16_384).filter(|&x| x % 3 == 0).collect();
    (pts, Vec::new())
}

// A single full container — single-interval RunStore. Used for the 318366d
// witness: `advance_back_to(last)` + `nth(n > u16::MAX)` delegates to
// `RunIter::nth` with a local `n` that exceeds `u16::MAX`.
fn single_full_container() -> (Vec<u32>, Vec<(u32, u16)>) {
    (Vec::new(), vec![(0, u16::MAX)])
}

#[test]
fn witness_iter_matches_model_case_two_full_containers() {
    // Forward iter on container 0 hits fwd_offset=u16::MAX on the last
    // element; move_next must consume that interval and reset to 0 before
    // yielding container 1's first element. Catches a24ff69_1 and c41bab3_1.
    // Backward iter on container 1 is symmetric — catches a24ff69_2 / c41bab3_2.
    let (vs, rs) = two_full_containers();
    expect_pass(
        property_iter_matches_model(vs, rs),
        "iter vs BTreeSet on two-full-container bitmap",
    );
}

#[test]
fn witness_advance_to_past_end_case_compressed_run() {
    // RunIter path: back pointer untouched, advance_to past the last element.
    let (vs, rs) = compressed_run_ranges();
    expect_pass(
        property_advance_to_matches_model(vs, rs, u32::MAX, 0x35B01),
        "advance_to past end of compressed-run RunStore",
    );
    // BitmapIter path: advance_back_to(100) THEN advance_to(300) on a dense
    // non-run container — exercises bitmap_advance_past_back_eaccd09.
    let (vs, rs) = bitmap_store_spec();
    expect_pass(
        property_advance_to_matches_model(vs, rs, 100, 300),
        "advance_back_to(100) + advance_to(300) on BitmapStore container",
    );
}

#[test]
fn witness_advance_back_to_before_start_case_compressed_run() {
    // RunIter path: advance_back_to before the first element.
    let (vs, rs) = compressed_run_ranges();
    expect_pass(
        property_advance_back_to_matches_model(vs, rs, 0, 499),
        "advance_back_to before start of compressed-run RunStore",
    );
    // BitmapIter path: advance_to(300) THEN advance_back_to(100).
    let (vs, rs) = bitmap_store_spec();
    expect_pass(
        property_advance_back_to_matches_model(vs, rs, 300, 100),
        "advance_to(300) + advance_back_to(100) on BitmapStore container",
    );
}

#[test]
fn witness_iter_nth_case_nth_over_u16_max() {
    // The 318366d trigger: single full-container RunStore, advance_back_to(last),
    // nth(n > u16::MAX). On base the guard in `RunIter::nth` returns None. On
    // the variant the guard is missing, the loop runs and yields a wrong Some.
    let (vs, rs) = single_full_container();
    expect_pass(
        property_iter_nth_matches_model(vs, rs, 65_535, 100_000),
        "advance_back_to(last) + nth(100_000) on full RunStore container",
    );
}

#[test]
fn witness_range_cardinality_case_full_container_range() {
    let (vs, rs) = single_full_container();
    expect_pass(
        property_range_cardinality_matches_model(vs, rs, 64, 6400),
        "range_cardinality slice on RunStore full container",
    );
}
