# Releases

## v0.3.8 (??/??/????)

- New features/improvements:

  - New function: `init_complex_matrix_from_slice()`

## v0.3.7 (08/09/2023)

- New features/improvements:

  - New method: `ComplexMatrixN::num_qubits()`

- API breaking changes:

  - Remove const generic parameter `N` from `Qureg`
  - Delete function: `create_*_qureg()`
  - Rename methods:
    - `Qureg::get_num_qubits()` to `num_qubits()`
    - `Qureg::get_num_amps_total()` to `num_amps_total()`

## v0.3.6 (23/08/2023)

- New features/improvements:

  - Add methods `create_qureg()`, `create_density_qureg()`

- API breaking changes:

  - Change `Qureg` type def. to `Qureg<'\_, const N: usize>`
    - Move many, many functions onto Qureg type
  - Remove redundant methods:
    - `Qureg::num_qubits_represented()`
    - `Qureg::get_num_amps()`

## v0.3.5 (05/08/2023)

- Change git repository name to: `quest_bind`.
- Implement Sync and Send traits for `QuestEnv` and `Qureg`.
- Rewrite exception handling using `panic` mechanism. No more global API locks!
  Our wrapper is now fully concurrent and much faster.
- Remove spurious checks that existed due to imperfect error catching.
- Fix bug: `calc_prob_of_all_outcomes()` didn't check properly for the length of
  the passed slice.

- Add method: `Qureg::get_num_amps_total()`

- API breaking changes:

  - Move free functions onto `QuestEnv` type:

    - `report_quest_env()`
    - `get_environment_string()`

  - Move free functions onto `Qureg` type:

    - `get_num_qubits()`
    - `get_num_amps()`
    - `report_qureg_params()`
    - `report_state()`
    - `report_state_to_screen()` (signature change)

  - Remove `QuestError` variants not needed anymore:

    - `NegativeProbability`
    - `NotDensityMatrix`
    - `QubitIndexError`

  - Function signature change:

    - `set_amps()`: parameter `num_amps` was redundant
    - `set_density_amps()`: parameter `num_amps` was redundant

## v0.3.4 (29/07/2023)

- Improve documentation
- Change returned error type in `rotate_{x,y,z}()` to `QubitIndexError`
- Internal exception handler passes error messages via a lock-free channel
  (added dependency: crossbeam, to be able to share between threads the global
  channel's receiver end -- the Standard Library's channel is not Sync).

## v0.3.3 (18/07/2023)

- Simplify handling QuEST exceptions
- Implement `Sync` and `Send` traits for QuestEnv
- Expand and improve documentation

## v0.3.2 (09/07/2023)

- Expand and improve documentation
- Various bug fixes

## v0.3.1 (02/07/2023)

- Publish to [crates.io](https://crates.io/crates/quest_bind)

## v0.3.0 (02/07/2023)

New features/improvements:

- Expanded and improved documentation and test suite.

API breaking changes:

- Change signature of the following functions:

  - `mix_nontp_kraus_map()`
  - `mix_nontp_two_qubit_kraus_map()`
  - `mix_nontp_multi_qubit_kraus_map()`

  These functions now take the list of Kraus operators by reference.

  - `apply_named_phase_func()`

  This function returns now `Result<(), QuestError>`.

  - `apply_pauli_sum()`
  - `apply_pauli_hamil()`

  These functions take argument `in_qureg` as `&mut` now (instead of a shared
  reference).

- Fix typo in the function name: `apply_trotter_circuit()`

- Function: `multi_controlled_multi_rotate_pauli()` also changes signature.

## v0.2.1 (01/07/2023)

New features/improvements:

- Expand and improve documentation and test suite

## v0.2.0 (24/06/2023)

New features/improvements:

- Improve documentation
- Catch exceptions thrown by QuEST
- Add build script
- Constructors/destructors for QuEST structs
- Add example: `grovers_search.rs`
- Use `Complex<f64>` type from `num` crate (as `QComplex`)
- Use compile flag `"f32"` to set floating point precision
- Add Github workflows CT

### v0.1.0 (11/06/2023)

Initial release.
