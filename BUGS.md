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
| `run_iter_forward_overflow_a24ff69_1` | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` |
| `run_iter_backward_overflow_a24ff69_2` | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` |
| `run_iter_forward_offset_reset_c41bab3_1` | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` |
| `run_iter_backward_offset_reset_c41bab3_2` | `property_iter_nth_back_matches_model` | `witness_iter_nth_back_case_cross_interval` |
| `bitmap_advance_past_back_eaccd09_1` | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` |
| `bitmap_advance_back_to_invariant_136b8f1_1` | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` |
| `run_iter_advance_to_past_end_3116bcc_1` | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` |
| `run_iter_advance_back_to_past_start_3116bcc_2` | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` |
| `nth_over_u16_max_318366d_1` | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_iter_matches_model` | OK | OK | OK | OK |
| `property_advance_to_matches_model` | OK | OK | OK | OK |
| `property_advance_back_to_matches_model` | OK | OK | OK | OK |
| `property_iter_nth_matches_model` | OK | OK | OK | OK |
| `property_iter_nth_back_matches_model` | OK | OK | OK | OK |
| `property_range_cardinality_matches_model` | OK | OK | OK | OK |

## Bug Details

### 1. run_iter_forward_overflow (a24ff69_1)
- **Variant**: `run_iter_forward_overflow_a24ff69_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next`
- **Property**: `property_iter_matches_model`
- **Witness**: `witness_iter_matches_model_case_full_bitmap`
- **Fix commit**: `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf` — "fix: interval store iterator not stopping"
- **Invariant violated**: `rb.iter().collect::<Vec<_>>()` must equal the sorted `BTreeSet` model.
- **How the mutation triggers**: When `forward_offset` is about to overflow `u16`, the fixed code advances `self.intervals.next()` before returning; the bug drops that call, so a dense container (e.g. `0..=65535` encoded as a single run) produces a repeating / truncated forward iteration that diverges from the set model on the final element.

### 2. run_iter_backward_overflow (a24ff69_2)
- **Variant**: `run_iter_backward_overflow_a24ff69_2`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next_back`
- **Property**: `property_iter_matches_model` (via `.rev()`)
- **Witness**: `witness_iter_matches_model_case_full_bitmap`
- **Fix commit**: `a24ff696b6e3cacd13479ab4358bba6f4ad02cdf`
- **Invariant violated**: Reversed iteration `rb.iter().rev()` must equal the reversed sorted model.
- **How the mutation triggers**: Symmetric to variant 1: the `backward_offset` overflow branch fails to call `self.intervals.next_back()`, so the back pointer never escapes the final interval.

### 3. run_iter_forward_offset_reset (c41bab3_1)
- **Variant**: `run_iter_forward_offset_reset_c41bab3_1`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next`
- **Property**: `property_iter_nth_matches_model`
- **Witness**: `witness_iter_nth_case_cross_interval`
- **Fix commit**: `c41bab3f0eb50b26b25cef4c066608add6c63f01` — "fix: when consuming a full run, reset the current offset"
- **Invariant violated**: After `iter.nth(k)`, the remaining forward iteration must equal `sorted.skip(k+1)`.
- **How the mutation triggers**: When a full run is consumed the fixed code resets `forward_offset = 0` before returning; the bug leaves the stale offset in place, so subsequent `next()` calls start mid-interval and the observable sequence skips elements.

### 4. run_iter_backward_offset_reset (c41bab3_2)
- **Variant**: `run_iter_backward_offset_reset_c41bab3_2`
- **Location**: `roaring/src/bitmap/store/interval_store.rs`, `RunIter::move_next_back`
- **Property**: `property_iter_nth_back_matches_model`
- **Witness**: `witness_iter_nth_back_case_cross_interval`
- **Fix commit**: `c41bab3f0eb50b26b25cef4c066608add6c63f01`
- **Invariant violated**: After `iter.nth_back(k)`, the remaining backward iteration must equal `sorted.rev().skip(k+1)`.
- **How the mutation triggers**: Symmetric to variant 3 on the backward path.

### 5. bitmap_advance_past_back (eaccd09_1)
- **Variant**: `bitmap_advance_past_back_eaccd09_1`
- **Location**: `roaring/src/bitmap/store/bitmap_store.rs`, `BitmapIter::advance_to`
- **Property**: `property_advance_to_matches_model`
- **Witness**: `witness_advance_to_past_end_case_compressed_run`
- **Fix commit**: `eaccd090783fae53124c054b88afa953a31a83af` — "fix: correctly empty bitmap container iter when advancing past the back"
- **Invariant violated**: `iter.advance_to(n)` for `n > rb.max()` must leave the iterator empty.
- **How the mutation triggers**: The fix sets `self.key = self.key_back` and `self.value = 0` to empty the iterator when `new_key > key_back`; the bug only clears `value_back`, leaving the front pointer on a stale key so later `collect()` replays it.

### 6. bitmap_advance_back_to_invariant (136b8f1_1)
- **Variant**: `bitmap_advance_back_to_invariant_136b8f1_1`
- **Location**: `roaring/src/bitmap/store/bitmap_store.rs`, `BitmapIter::advance_back_to`
- **Property**: `property_advance_back_to_matches_model`
- **Witness**: `witness_advance_back_to_before_start_case_compressed_run`
- **Fix commit**: `136b8f1e7a4807a0662e07646b8a03c1beb7b06d` — "fix: bitmap advance_back_to could violate invariants"
- **Invariant violated**: `iter.advance_back_to(n)` for `n < rb.min()` must leave the iterator empty — in particular, `key_back >= key` must continue to hold.
- **How the mutation triggers**: The fix returns early with a fully-reset state when `new_key < key`; the bug falls through to `(0, &mut self.value)`, writing a 0 value into the front word while leaving `key_back` ahead of `key`. Reversed iteration then produces a stale element.

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
- **Witness**: `witness_iter_nth_case_cross_interval`
- **Fix commit**: `318366d041d958d59bef3e51569ecb452a1ea893` — "fix: nth where n > u16::MAX is now correct"
- **Invariant violated**: `iter.nth(k)` must match `sorted.get(k).copied()` for arbitrary `k`.
- **How the mutation triggers**: The fix short-circuits the loop when `n > u16::MAX`; the bug keeps the loop, which internally narrows to `u16` and wraps, effectively performing `nth(n % 2^16)` instead of returning `None` or the correct element. For the multi-interval witness the in-range `k = 150` still triggers the bug through a different path in the loop prologue that relied on the early-return.
