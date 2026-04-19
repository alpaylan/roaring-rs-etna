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
| `property_iter_nth_back_matches_model` | `IterNthBackMatchesModel` |
| `property_range_cardinality_matches_model` | `RangeCardinalityMatchesModel` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `run_iter_forward_overflow_a24ff69_1` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 002 | `run_iter_forward_overflow_a24ff69_1` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 003 | `run_iter_forward_overflow_a24ff69_1` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 004 | `run_iter_forward_overflow_a24ff69_1` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 005 | `run_iter_backward_overflow_a24ff69_2` | proptest | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- proptest IterMatchesModel` |
| 006 | `run_iter_backward_overflow_a24ff69_2` | quickcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- quickcheck IterMatchesModel` |
| 007 | `run_iter_backward_overflow_a24ff69_2` | crabcheck | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- crabcheck IterMatchesModel` |
| 008 | `run_iter_backward_overflow_a24ff69_2` | hegel | `property_iter_matches_model` | `witness_iter_matches_model_case_full_bitmap` | `cargo run --release --bin etna -- hegel IterMatchesModel` |
| 009 | `run_iter_forward_offset_reset_c41bab3_1` | proptest | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- proptest IterNthMatchesModel` |
| 010 | `run_iter_forward_offset_reset_c41bab3_1` | quickcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- quickcheck IterNthMatchesModel` |
| 011 | `run_iter_forward_offset_reset_c41bab3_1` | crabcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- crabcheck IterNthMatchesModel` |
| 012 | `run_iter_forward_offset_reset_c41bab3_1` | hegel | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- hegel IterNthMatchesModel` |
| 013 | `run_iter_backward_offset_reset_c41bab3_2` | proptest | `property_iter_nth_back_matches_model` | `witness_iter_nth_back_case_cross_interval` | `cargo run --release --bin etna -- proptest IterNthBackMatchesModel` |
| 014 | `run_iter_backward_offset_reset_c41bab3_2` | quickcheck | `property_iter_nth_back_matches_model` | `witness_iter_nth_back_case_cross_interval` | `cargo run --release --bin etna -- quickcheck IterNthBackMatchesModel` |
| 015 | `run_iter_backward_offset_reset_c41bab3_2` | crabcheck | `property_iter_nth_back_matches_model` | `witness_iter_nth_back_case_cross_interval` | `cargo run --release --bin etna -- crabcheck IterNthBackMatchesModel` |
| 016 | `run_iter_backward_offset_reset_c41bab3_2` | hegel | `property_iter_nth_back_matches_model` | `witness_iter_nth_back_case_cross_interval` | `cargo run --release --bin etna -- hegel IterNthBackMatchesModel` |
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
| 033 | `nth_over_u16_max_318366d_1` | proptest | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- proptest IterNthMatchesModel` |
| 034 | `nth_over_u16_max_318366d_1` | quickcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- quickcheck IterNthMatchesModel` |
| 035 | `nth_over_u16_max_318366d_1` | crabcheck | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- crabcheck IterNthMatchesModel` |
| 036 | `nth_over_u16_max_318366d_1` | hegel | `property_iter_nth_matches_model` | `witness_iter_nth_case_cross_interval` | `cargo run --release --bin etna -- hegel IterNthMatchesModel` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_iter_matches_model_case_full_bitmap` — `property_iter_matches_model((0..=65535).collect())` → `Pass` on base. A dense full container is encoded as a single run; any bug in `RunIter`'s forward/backward offset arithmetic diverges from the sorted-set model on the final element.
- `witness_advance_to_past_end_case_compressed_run` — `property_advance_to_matches_model(compressed_run_bitmap, 0x35B01)` → `Pass` on base. `advance_to` past every element must empty both the forward and backward iterators.
- `witness_advance_back_to_before_start_case_compressed_run` — `property_advance_back_to_matches_model(compressed_run_bitmap, 499)` → `Pass` on base. Symmetric backward form.
- `witness_iter_nth_case_cross_interval` — `property_iter_nth_matches_model(multi_interval_bitmap, 150)` → `Pass` on base. `nth(150)` crosses interval boundaries and requires that forward offsets reset when a run is fully consumed.
- `witness_iter_nth_back_case_cross_interval` — `property_iter_nth_back_matches_model(multi_interval_bitmap, 150)` → `Pass` on base. Symmetric backward form.
- `witness_range_cardinality_case_full_container_range` — `property_range_cardinality_matches_model((0..=65535).collect(), 64, 6400)` → `Pass` on base. Range cut across a dense container exercises per-word loop indices.
