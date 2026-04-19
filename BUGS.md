# roaring-rs — Injected Bugs

Total mutations: 9

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `run_iter_forward_overflow` | `run_iter_forward_overflow_a24ff69_1` | `patches/run_iter_forward_overflow_a24ff69_1.patch` | `patch` | `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf` |
| 2 | `run_iter_backward_overflow` | `run_iter_backward_overflow_a24ff69_2` | `patches/run_iter_backward_overflow_a24ff69_2.patch` | `patch` | `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf` |
| 3 | `run_iter_forward_offset_reset` | `run_iter_forward_offset_reset_c41bab3_1` | `patches/run_iter_forward_offset_reset_c41bab3_1.patch` | `patch` | `c41bab3f0eb50b26b25cef4c066608add6c63f01` |
| 4 | `run_iter_backward_offset_reset` | `run_iter_backward_offset_reset_c41bab3_2` | `patches/run_iter_backward_offset_reset_c41bab3_2.patch` | `patch` | `c41bab3f0eb50b26b25cef4c066608add6c63f01` |
| 5 | `bitmap_advance_past_back` | `bitmap_advance_past_back_eaccd09_1` | `patches/bitmap_advance_past_back_eaccd09_1.patch` | `patch` | `eaccd090783fae53124c054b88afa953a31a83af` |
| 6 | `bitmap_advance_back_to_invariant` | `bitmap_advance_back_to_invariant_136b8f1_1` | `patches/bitmap_advance_back_to_invariant_136b8f1_1.patch` | `patch` | `136b8f1e7a4807a0662e07646b8a03c1beb7b06d` |
| 7 | `run_iter_advance_to_past_end` | `run_iter_advance_to_past_end_3116bcc_1` | `patches/run_iter_advance_to_past_end_3116bcc_1.patch` | `patch` | `3116bccab187a639238cd50754935400a433a3c5` |
| 8 | `run_iter_advance_back_to_past_start` | `run_iter_advance_back_to_past_start_3116bcc_2` | `patches/run_iter_advance_back_to_past_start_3116bcc_2.patch` | `patch` | `3116bccab187a639238cd50754935400a433a3c5` |
| 9 | `nth_over_u16_max` | `nth_over_u16_max_318366d_1` | `patches/nth_over_u16_max_318366d_1.patch` | `patch` | `318366d041d958d59bef3e51569ecb452a1ea893` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `run_iter_forward_overflow_a24ff69_1` | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` |
| `run_iter_backward_overflow_a24ff69_2` | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` |
| `run_iter_forward_offset_reset_c41bab3_1` | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` |
| `run_iter_backward_offset_reset_c41bab3_2` | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` |
| `bitmap_advance_past_back_eaccd09_1` | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` |
| `bitmap_advance_back_to_invariant_136b8f1_1` | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` |
| `run_iter_advance_to_past_end_3116bcc_1` | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` |
| `run_iter_advance_back_to_past_start_3116bcc_2` | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` |
| `nth_over_u16_max_318366d_1` | `property_iter_nth_matches_model` | `witness_iter_nth_case_nth_over_u16_max` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_iter_matches_model` | OK | OK | OK | OK |
| `property_advance_to_matches_model` | OK | OK | OK | OK |
| `property_advance_back_to_matches_model` | OK | OK | OK | OK |
| `property_iter_nth_matches_model` | OK | OK | OK | OK |
| `property_range_cardinality_matches_model` | OK | OK | OK | OK |

## Bug Details

### 1. run_iter_forward_overflow (a24ff69_1)
- **Variant**: `run_iter_forward_overflow_a24ff69_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next`
- **Property**: `property_iter_matches_model`
- **Witness**: `witness_iter_matches_model_case_two_full_containers`
- **Fix commit**: `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf` — "fix: interval store iterator not stopping"
- **Invariant violated**: `rb.iter().collect::<Vec<_>>()` must equal the sorted `BTreeSet` model.
- **How the mutation triggers**: When `forward_offset` is about to overflow `u16`, the fixed code advances `self.intervals.next()` before returning; the bug drops that call, so two back-to-back full-container runs produce a repeating / truncated forward iteration that diverges from the set model at the container boundary.

### 2. run_iter_backward_overflow (a24ff69_2)
- **Variant**: `run_iter_backward_overflow_a24ff69_2`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next_back`
- **Property**: `property_iter_matches_model` (via `.rev()`)
- **Witness**: `witness_iter_matches_model_case_two_full_containers`
- **Fix commit**: `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf`
- **Invariant violated**: Reversed iteration `rb.iter().rev()` must equal the reversed sorted model.
- **How the mutation triggers**: Symmetric to variant 1: the `backward_offset` overflow branch fails to call `self.intervals.next_back()`, so the back pointer never escapes the final interval.

### 3. run_iter_forward_offset_reset (c41bab3_1)
- **Variant**: `run_iter_forward_offset_reset_c41bab3_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next`
- **Property**: `property_iter_matches_model`
- **Witness**: `witness_iter_matches_model_case_two_full_containers`
- **Fix commit**: `c41bab3f0eb50b26b25cef4c066608add6c63f01` — "fix: when consuming a full run, reset the current offset"
- **Invariant violated**: After `Iter::nth(k)` lands on a container boundary the cached inner `RunIter` must hold `forward_offset == 0`; otherwise the subsequent `len()` fails `debug_assert!(total_size >= total_offset)` in `RunIter::remaining_size`.
- **How the mutation triggers**: When a full run is consumed the fixed code resets `forward_offset = 0` before returning; the bug leaves the stale `u16::MAX` offset in place. The witness probes `nth(65_535) + len()` on a two-full-container bitmap, which stores the first container's `RunIter` in `Iter::front` with the stale offset; `len()` then panics. Requires `[profile.release] debug-assertions = true`.

### 4. run_iter_backward_offset_reset (c41bab3_2)
- **Variant**: `run_iter_backward_offset_reset_c41bab3_2`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next_back`
- **Property**: `property_iter_matches_model`
- **Witness**: `witness_iter_matches_model_case_two_full_containers`
- **Fix commit**: `c41bab3f0eb50b26b25cef4c066608add6c63f01`
- **Invariant violated**: Symmetric to variant 3 on the backward path.
- **How the mutation triggers**: Witness probes `nth_back(65_535) + len()`; the stale `backward_offset` crashes `debug_assert!` in `remaining_size`.

### 5. bitmap_advance_past_back (eaccd09_1)
- **Variant**: `bitmap_advance_past_back_eaccd09_1`
- **Location**: `roaring/src/bitmap/store/bitmap_store.rs`, `BitmapIter::advance_to`
- **Property**: `property_advance_to_matches_model`
- **Witness**: `witness_advance_to_past_end_case_compressed_run`
- **Fix commit**: `eaccd090783fae53124c054b88afa953a31a83af` — "fix: correctly empty bitmap container iter when advancing past the back"
- **Invariant violated**: After `advance_back_to(b)` + `advance_to(a)` with `a > b`, the bitmap iterator must be empty.
- **How the mutation triggers**: The fix sets `self.key = self.key_back` and `self.value = 0` when `new_key > key_back`; the bug only clears `value_back`, leaving the front pointer on a stale key so later `collect()` replays it. The BitmapStore leg of the witness (`advance_back_to(100) + advance_to(300)` over a dense container) exposes this.

### 6. bitmap_advance_back_to_invariant (136b8f1_1)
- **Variant**: `bitmap_advance_back_to_invariant_136b8f1_1`
- **Location**: `roaring/src/bitmap/store/bitmap_store.rs`, `BitmapIter::advance_back_to`
- **Property**: `property_advance_back_to_matches_model`
- **Witness**: `witness_advance_back_to_before_start_case_compressed_run`
- **Fix commit**: `136b8f1e7a4807a0662e07646b8a03c1beb7b06d` — "fix: bitmap advance_back_to could violate invariants"
- **Invariant violated**: After `advance_to(f)` + `advance_back_to(b)` with `b < f` the `key_back >= key` invariant must still hold.
- **How the mutation triggers**: The fix returns early with a fully-reset state when `new_key < key`; the bug falls through to `(0, &mut self.value)`, writing a 0 value into the front word while leaving `key_back` ahead of `key`. The BitmapStore leg of the witness (`advance_to(300) + advance_back_to(100)`) trips a `debug_assert!` in subsequent iteration. Requires `[profile.release] debug-assertions = true`.

### 7. run_iter_advance_to_past_end (3116bcc_1)
- **Variant**: `run_iter_advance_to_past_end_3116bcc_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::advance_to`
- **Property**: `property_advance_to_matches_model`
- **Witness**: `witness_advance_to_past_end_case_compressed_run`
- **Fix commit**: `3116bccab187a639238cd50754935400a433a3c5` — "fix: when advancing past the begin/end of a range container's iter, consume the whole thing"
- **Invariant violated**: `iter.advance_to(n)` past every element of a run-encoded container must yield an empty iterator.
- **How the mutation triggers**: The fix calls `self.intervals.nth(index)` when `index == len`, consuming the whole underlying slice iterator; the bug returns without touching it, so the iterator still reports remaining intervals and replays them on subsequent `next()`.

### 8. run_iter_advance_back_to_past_start (3116bcc_2)
- **Variant**: `run_iter_advance_back_to_past_start_3116bcc_2`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::advance_back_to`
- **Property**: `property_advance_back_to_matches_model`
- **Witness**: `witness_advance_back_to_before_start_case_compressed_run`
- **Fix commit**: `3116bccab187a639238cd50754935400a433a3c5`
- **Invariant violated**: Symmetric to variant 7 on the backward path.
- **How the mutation triggers**: Same omission in the `advance_back_to` Err(index == 0) branch.

### 9. nth_over_u16_max (318366d_1)
- **Variant**: `nth_over_u16_max_318366d_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `<RunIter as Iterator>::nth`
- **Property**: `property_iter_nth_matches_model`
- **Witness**: `witness_iter_nth_case_nth_over_u16_max`
- **Fix commit**: `318366d041d958d59bef3e51569ecb452a1ea893` — "fix: nth where n > u16::MAX is now correct"
- **Invariant violated**: `iter.nth(k)` on a single full-container RunStore with `k > u16::MAX` must return `None`.
- **How the mutation triggers**: The fix short-circuits the loop when `n > u16::MAX`; the bug keeps the loop, which internally narrows to `u16` and wraps, effectively performing `nth(n % 2^16)` and returning the wrong `Some`. The witness uses `advance_back_to(u16::MAX)` + `nth(100_000)` on a single full container so the `nth` call reaches `RunIter::nth` with `n > u16::MAX`.
