# roaring-rs — ETNA Tasks

Total tasks: 36

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task. The `<PropertyKey>` token in the command column uses the PascalCase key recognised by `roaring/src/bin/etna.rs`; passing `All` runs every property for the named framework in a single invocation.

## Property keys

| Property | PropertyKey |
|----------|-------------|
| `property_iter_matches_model` | `IterMatchesModel` |
| `property_advance_to_matches_model` | `AdvanceToMatchesModel` |
| `property_advance_back_to_matches_model` | `AdvanceBackToMatchesModel` |
| `property_iter_nth_matches_model` | `IterNthMatchesModel` |
| `property_range_cardinality_matches_model` | `RangeCardinalityMatchesModel` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `run_iter_forward_overflow_a24ff69_1` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 002 | `run_iter_forward_overflow_a24ff69_1` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 003 | `run_iter_forward_overflow_a24ff69_1` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 004 | `run_iter_forward_overflow_a24ff69_1` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 005 | `run_iter_backward_overflow_a24ff69_2` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 006 | `run_iter_backward_overflow_a24ff69_2` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 007 | `run_iter_backward_overflow_a24ff69_2` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 008 | `run_iter_backward_overflow_a24ff69_2` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 009 | `run_iter_forward_offset_reset_c41bab3_1` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 010 | `run_iter_forward_offset_reset_c41bab3_1` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 011 | `run_iter_forward_offset_reset_c41bab3_1` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 012 | `run_iter_forward_offset_reset_c41bab3_1` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 013 | `run_iter_backward_offset_reset_c41bab3_2` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 014 | `run_iter_backward_offset_reset_c41bab3_2` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 015 | `run_iter_backward_offset_reset_c41bab3_2` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 016 | `run_iter_backward_offset_reset_c41bab3_2` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_two_full_containers` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 017 | `bitmap_advance_past_back_eaccd09_1` | proptest | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- proptest AdvanceToMatchesModel` |
| 018 | `bitmap_advance_past_back_eaccd09_1` | quickcheck | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- quickcheck AdvanceToMatchesModel` |
| 019 | `bitmap_advance_past_back_eaccd09_1` | crabcheck | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- crabcheck AdvanceToMatchesModel` |
| 020 | `bitmap_advance_past_back_eaccd09_1` | hegel | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- hegel AdvanceToMatchesModel` |
| 021 | `bitmap_advance_back_to_invariant_136b8f1_1` | proptest | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- proptest AdvanceBackToMatchesModel` |
| 022 | `bitmap_advance_back_to_invariant_136b8f1_1` | quickcheck | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- quickcheck AdvanceBackToMatchesModel` |
| 023 | `bitmap_advance_back_to_invariant_136b8f1_1` | crabcheck | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- crabcheck AdvanceBackToMatchesModel` |
| 024 | `bitmap_advance_back_to_invariant_136b8f1_1` | hegel | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- hegel AdvanceBackToMatchesModel` |
| 025 | `run_iter_advance_to_past_end_3116bcc_1` | proptest | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- proptest AdvanceToMatchesModel` |
| 026 | `run_iter_advance_to_past_end_3116bcc_1` | quickcheck | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- quickcheck AdvanceToMatchesModel` |
| 027 | `run_iter_advance_to_past_end_3116bcc_1` | crabcheck | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- crabcheck AdvanceToMatchesModel` |
| 028 | `run_iter_advance_to_past_end_3116bcc_1` | hegel | `property_advance_to_matches_model` | `witness_advance_to_past_end_case_compressed_run` | `cargo run --release --bin etna -- hegel AdvanceToMatchesModel` |
| 029 | `run_iter_advance_back_to_past_start_3116bcc_2` | proptest | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- proptest AdvanceBackToMatchesModel` |
| 030 | `run_iter_advance_back_to_past_start_3116bcc_2` | quickcheck | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- quickcheck AdvanceBackToMatchesModel` |
| 031 | `run_iter_advance_back_to_past_start_3116bcc_2` | crabcheck | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- crabcheck AdvanceBackToMatchesModel` |
| 032 | `run_iter_advance_back_to_past_start_3116bcc_2` | hegel | `property_advance_back_to_matches_model` | `witness_advance_back_to_before_start_case_compressed_run` | `cargo run --release --bin etna -- hegel AdvanceBackToMatchesModel` |
| 033 | `nth_over_u16_max_318366d_1` | proptest | `property_iter_nth_matches_model` | `witness_iter_nth_case_nth_over_u16_max` | `cargo run --release --bin etna -- proptest IterNthMatchesModel` |
| 034 | `nth_over_u16_max_318366d_1` | quickcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_nth_over_u16_max` | `cargo run --release --bin etna -- quickcheck IterNthMatchesModel` |
| 035 | `nth_over_u16_max_318366d_1` | crabcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_nth_over_u16_max` | `cargo run --release --bin etna -- crabcheck IterNthMatchesModel` |
| 036 | `nth_over_u16_max_318366d_1` | hegel | `property_iter_nth_matches_model` | `witness_iter_nth_case_nth_over_u16_max` | `cargo run --release --bin etna -- hegel IterNthMatchesModel` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_iter_matches_model_case_two_full_containers` — `property_iter_matches_model((), [(0, u16::MAX), (65_536, u16::MAX)])` → `Pass` on base. Two adjacent full-container runs expose both `RunIter` overflow branches (a24ff69_1/_2) on the container boundary and, via a post-sequence `nth(k) + len()` probe at container boundaries, the stale forward/backward offset debug-assertion failures (c41bab3_1/_2). Requires `debug-assertions = true` in release.
- `witness_advance_to_past_end_case_compressed_run` — `property_advance_to_matches_model(compressed_run_bitmap, u32::MAX, 0x35B01)` + a second invocation on a `BitmapStore` spec (`advance_back_to(100) + advance_to(300)`) → `Pass` on base. The RunStore invocation catches 3116bcc_1; the BitmapStore invocation catches eaccd09_1.
- `witness_advance_back_to_before_start_case_compressed_run` — `property_advance_back_to_matches_model(compressed_run_bitmap, 0, 499)` + a `BitmapStore` invocation (`advance_to(300) + advance_back_to(100)`) → `Pass` on base. Catches 3116bcc_2 on the RunStore path and 136b8f1_1 on the BitmapStore path.
- `witness_iter_nth_case_nth_over_u16_max` — `property_iter_nth_matches_model(single_full_container, 65_535, 100_000)` → `Pass` on base. `advance_back_to(last)` followed by `nth(n > u16::MAX)` is dispatched straight into `RunIter::nth` with `n > u16::MAX`. The base 318366d guard returns `None`; the variant loops and yields the wrong `Some`.
- `witness_range_cardinality_case_full_container_range` — `property_range_cardinality_matches_model(single_full_container, 64, 6400)` → `Pass` on base. Kept as a smoke test for the `range_cardinality` path; no current variant targets it.
