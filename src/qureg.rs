use std::ffi::CString;

use super::{
    catch_quest_exception,
    ffi,
    BitEncoding,
    ComplexMatrix2,
    ComplexMatrix4,
    ComplexMatrixN,
    PauliHamil,
    PauliOpType,
    PhaseFunc,
    Qcomplex,
    Qreal,
    QuestEnv,
    QuestError,
    Vector,
};

#[derive(Debug)]
pub struct Qureg<'a> {
    pub(crate) env: &'a QuestEnv,
    pub(crate) reg: ffi::Qureg,
}

impl<'a> Qureg<'a> {
    /// Creates a state-vector Qureg object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`QuestError::InvalidQuESTInputError`](crate::QuestError::InvalidQuESTInputError)
    /// on failure.  This is an exception thrown by `QuEST`.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn try_new(
        num_qubits: i32,
        env: &'a QuestEnv,
    ) -> Result<Self, QuestError> {
        Ok(Self {
            env,
            reg: catch_quest_exception(|| unsafe {
                ffi::createQureg(num_qubits, env.0)
            })?,
        })
    }

    ///  Creates a density matrix Qureg object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`QuestError::InvalidQuESTInputError`](crate::QuestError::InvalidQuESTInputError)
    /// on failure.  This is an exception thrown by `QuEST`.
    ///
    /// [Quest API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn try_new_density(
        num_qubits: i32,
        env: &'a QuestEnv,
    ) -> Result<Self, QuestError> {
        Ok(Self {
            env,
            reg: catch_quest_exception(|| unsafe {
                ffi::createDensityQureg(num_qubits, env.0)
            })?,
        })
    }

    #[must_use]
    pub fn is_density_matrix(&self) -> bool {
        self.reg.isDensityMatrix != 0
    }

    /// Print the current state vector of probability amplitudes to file.
    ///
    /// ## File format:
    ///
    /// ```text
    /// real, imag
    /// realComponent1, imagComponent1
    /// realComponent2, imagComponent2
    /// ...
    /// realComponentN, imagComponentN
    /// ```
    ///
    ///  ## File naming convention:
    ///
    /// For each node that the program runs on, a file
    /// `state_rank_[node_rank].csv` is generated. If there is  more than
    /// one node, ranks after the first do not include the header:
    ///
    /// ```text
    /// real, imag
    /// ```
    ///
    /// so that files are easier to combine.
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn report_state(&self) {
        catch_quest_exception(|| unsafe { ffi::reportState(self.reg) })
            .expect("report_state should never fail");
    }

    /// Print the current state vector of probability amplitudes.
    ///
    /// Print the current state vector of probability amplitudes for a set of
    /// qubits to standard out. For debugging purposes. Each rank should
    /// print output serially.  Only print output for systems <= 5 qubits.
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn report_state_to_screen(
        &self,
        report_rank: i32,
    ) {
        catch_quest_exception(|| unsafe {
            ffi::reportStateToScreen(self.reg, self.env.0, report_rank);
        })
        .expect("report_state_to screen should never fail");
    }

    /// Returns the number of qubits represented.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// assert_eq!(qureg.num_qubits(), 3);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub const fn num_qubits(&self) -> i32 {
        self.reg.numQubitsRepresented
    }

    /// Return the total number of amplitudes in the register.
    ///
    /// - If `Qureg` is a state-vector, this is equal to: `2^N`, where `N` is
    ///   the number of qubits in the register: [`num_qubits()`]
    /// - If `Qureg` is a density matrix, this is equal to `2^(2N)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// assert_eq!(qureg.num_amps_total(), 64);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub fn num_amps_total(&self) -> i64 {
        self.reg.numAmpsTotal
    }

    /// Report information about a set of qubits.
    ///
    /// This function prints to stdout: number of qubits, number of probability
    /// amplitudes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.report_qureg_params();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn report_qureg_params(&self) {
        catch_quest_exception(|| unsafe {
            ffi::reportQuregParams(self.reg);
        })
        .expect("report_qureg_params should never fail");
    }

    /// Initializes a `Qureg` to have all-zero-amplitudes.
    ///
    /// This is an unphysical state, useful for iteratively building a state
    /// with functions like
    /// [`set_weighted_qureg()`][api-set-weighted-qureg], and should
    /// not be confused with [`init_zero_state()`][api-init-zero-state].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_blank_state();
    ///
    /// assert!(qureg.get_prob_amp(0).unwrap().abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(1).unwrap().abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(2).unwrap().abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(3).unwrap().abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-set-weighted-qureg]: crate::set_weighted_qureg()
    /// [api-init-zero-state]: crate::Qureg::init_zero_state()
    /// [api-qureg]: crate::Qureg
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_blank_state(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::initBlankState(self.reg);
        })
        .expect("init_blank_state should always succeed");
    }

    /// Initialize `qureg` into the zero state.
    ///
    /// If `qureg` is a state-vector of `N` qubits, it is modified to state
    /// `|0>^{\otimes N}`.  If `qureg` is a density matrix of `N` qubits, it is
    /// modified to state `|0><0|^{\otimes N}`.
    ///
    /// # Parameters
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_zero_state();
    ///
    /// assert!((qureg.get_prob_amp(0).unwrap() - 1.).abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(1).unwrap().abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(2).unwrap().abs() < EPSILON);
    /// assert!(qureg.get_prob_amp(3).unwrap().abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-qureg]: crate::Qureg
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_zero_state(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::initZeroState(self.reg);
        })
        .expect("init_zero_state should always succeed");
    }

    /// Initialize `qureg` into the plus state.
    ///
    /// If `qureg` is a state-vector of `N` qubits, it is modified to state:
    ///
    /// ```latex
    ///   {| + \rangle}^{\otimes N} = \frac{1}{\sqrt{2^N}} (| 0 \rangle + | 1 \rangle)^{\otimes N}.
    /// ```
    ///
    /// If `qureg` is a density matrix of `N`, it is modified to state:
    ///
    /// ```latex
    ///   {| + \rangle\langle+|}^{\otimes N} = \frac{1}{{2^N}} \sum_i\sum_j |i\rangle\langle j|.
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_plus_state();
    ///
    /// assert!((qureg.get_prob_amp(0).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((qureg.get_prob_amp(1).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((qureg.get_prob_amp(2).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((qureg.get_prob_amp(3).unwrap() - 0.25).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-qureg]: crate::Qureg
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_plus_state(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::initPlusState(self.reg);
        })
        .expect("init_plus_state should always succeed");
    }

    /// Initialize `qureg` into a classical state.
    ///
    /// This state is also known as a "computational basis state" with index
    /// `state_ind`.
    ///
    /// If `qureg` is a state-vector, it will become: `|state_ind>`. If
    /// `qureg`is a density matrix, it will become:
    ///
    /// ```text
    ///   |state_ind> <state_ind|
    /// ````
    ///
    /// Classical states are indexed from zero, so that `state_ind=0` produces
    /// `|0..00>`,  and  `state_ind=1` produces `|00..01>`, and `state_ind=2^N -
    /// 1` produces `|11..11>`. Subsequent calls to
    /// [`get_prob_amp()`][api-get-prob-amp] will yield `0` for all indices
    /// except `state_ind`,  and the phase of `state_ind`'s amplitude will
    /// be `1` (real).
    ///
    /// This function can be used to initialise `qureg` into a specific binary
    /// state  (e.g. `11001`) using a binary literal.
    ///
    /// # Parameters
    ///
    ///  - `state_ind` the index of the basis state to modify `qureg` into
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `state_ind` is outside [0, qureg.[`num_amps_total()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_classical_state(8);
    /// let prob = qureg.get_prob_amp(0).unwrap();
    ///
    /// assert!((prob.abs() - 1.) < EPSILON);
    /// ```
    ///
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-get-prob-amp]: crate::Qureg::get_prob_amp()
    /// [`num_amps_total()`]: crate::Qureg::num_amps_total()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_classical_state(
        &mut self,
        state_ind: i64,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::initClassicalState(self.reg, state_ind);
        })
    }

    /// Initialize `qureg` into a pure state.
    ///
    /// - If `qureg` is a state-vector, this merely clones `pure` into `qureg`.
    /// - If `qureg` is a density matrix, this makes `qureg` 100% likely to be
    ///   in the `pure` state.
    ///
    /// # Parameters
    ///
    /// - `pure`: a state-vector containing the pure state into which to
    ///   initialize `qureg`
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `self` and `pure` have mismatching dimensions
    ///   - if `pure` is a density matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// let pure_state =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_pure_state(&pure_state).unwrap();
    ///
    /// assert!((qureg.calc_purity().unwrap() - 1.0).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_pure_state(
        &mut self,
        pure_: &Qureg<'_>,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::initPureState(self.reg, pure_.reg);
        })
    }

    /// Initialize `qureg` to be in a debug state.
    ///
    /// Set `qureg` to be in the un-normalized, non-physical state with
    /// with `n`th complex amplitude given by:
    ///
    /// ```text
    ///   2n/10 + i*(2n+1)/10.
    /// ```
    ///
    /// This is used internally for debugging and testing.
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_debug_state(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::initDebugState(self.reg);
        })
        .expect("init_debug_state() should always succeed");
    }

    /// Initialize `qureg` by specifying all amplitudes.
    ///
    /// For density matrices, it is assumed the amplitudes have been flattened
    /// column-wise into the given arrays.
    ///
    /// The real and imaginary components of the amplitudes are passed in
    /// separate arrays, `reals` and `imags`, each of which must have length
    /// [`qureg.num_amps_total()`]. There is no automatic checking that the
    /// passed arrays are L2 normalized, so this can be used to prepare `qureg`
    /// in a non-physical state.
    ///
    /// In distributed mode, this would require the complete state to fit in
    /// every node. To manually prepare a state for which all amplitudes cannot
    /// fit into a single node, use [`set_amps()`]
    ///
    /// # Parameters
    ///
    /// - `reals`: array of the real components of the new amplitudes
    /// - `imags`: array of the imaginary components of the new amplitudes
    ///
    /// # Errors
    ///
    /// - [`ArrayLengthError`],
    ///   - if either `reals` or `imags` have fewer than
    ///     [`qureg.num_amps_total()`] elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.init_state_from_amps(&[1., 0., 0., 0.], &[0., 0., 0., 0.]);
    /// let prob = qureg.get_prob_amp(0).unwrap();
    ///
    /// assert!((prob - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`qureg.num_amps_total()`]: crate::Qureg::num_amps_total()
    /// [`set_amps()`]: crate::Qureg::set_amps()
    /// [`ArrayLengthError`]: crate::QuestError::ArrayLengthError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_state_from_amps(
        &mut self,
        reals: &[Qreal],
        imags: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_amps_total = self.num_amps_total() as usize;
        if reals.len() < num_amps_total || imags.len() < num_amps_total {
            return Err(QuestError::ArrayLengthError);
        }
        catch_quest_exception(|| unsafe {
            ffi::initStateFromAmps(self.reg, reals.as_ptr(), imags.as_ptr());
        })
    }

    /// Overwrites a contiguous subset of the amplitudes in a state-vector.
    ///
    /// Only amplitudes with indices in `[start_ind,  start_ind + reals.len()]`
    /// will be changed. The resulting `qureg` may not necessarily be in an
    /// L2 normalized state.
    ///
    /// In distributed mode, this function assumes the subset `reals` and
    /// `imags` exist (at least) on the node containing the ultimately
    /// updated elements. For example, below is the correct way to modify
    /// the full 8 elements of `qureg` when split between 2 nodes:
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let re = &mut [1., 2., 3., 4.];
    /// let im = &mut [1., 2., 3., 4.];
    /// qureg.set_amps(0, re, im);
    ///
    /// // modify re and im to the next set of elements
    /// for i in 0..4 {
    ///     re[i] += 4.;
    ///     im[i] += 4.;
    /// }
    /// qureg.set_amps(4, re, im);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `start_ind`: the index of the first amplitude in `qureg` to modify
    /// - `reals`: array of the real components of the new amplitudes
    /// - `imags`: array of the imaginary components of the new amplitudes
    ///
    /// # Errors
    ///
    /// - [`ArrayLengthError`]
    ///   - if `reals.len()` and `imags.len()` are different
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if `qureg` is not a state-vector (i.e. is a density matrix)
    ///   - if `start_ind` is outside [0, [`qureg.num_amps_total()`]]
    ///   - if `reals.len()` is outside [0, `qureg.num_amps_total()`]
    ///   - if `reals.len()` + `start_ind` >= `qureg.num_amps_total()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let re = &[1., 2., 3.];
    /// let im = &[4., 5., 6.];
    /// let start_ind = 1;
    /// qureg.set_amps(start_ind, re, im);
    ///
    /// let amp = qureg.get_real_amp(3).unwrap();
    /// assert!((amp - 3.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`qureg.num_amps_total()`]: crate::Qureg::num_amps_total()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`ArrayLengthError`]: crate::QuestError::ArrayLengthError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn set_amps(
        &mut self,
        start_ind: i64,
        reals: &[Qreal],
        imags: &[Qreal],
    ) -> Result<(), QuestError> {
        if reals.len() != imags.len() {
            return Err(QuestError::ArrayLengthError);
        }
        let num_amps = reals.len() as i64;
        catch_quest_exception(|| unsafe {
            ffi::setAmps(
                self.reg,
                start_ind,
                reals.as_ptr(),
                imags.as_ptr(),
                num_amps,
            );
        })
    }

    /// Overwrites a contiguous subset of the amplitudes in a density-matrix.
    ///
    /// Only the first `reals.len()` amplitudes starting from row-column index
    /// `(start_row, start_col)`, and proceeding down the column (wrapping
    /// around between rows) will be modified. The resulting `qureg` may not
    /// necessarily be in an L2 normalized state.
    ///
    /// In distributed mode, this function assumes the subset `reals` and
    /// `imags` exist (at least) on the node containing the ultimately
    /// updated elements. See also [`set_amps()`] for more details.
    ///
    /// # Parameters
    ///
    /// - `start_row`: the row-index of the first amplitude in `qureg` to modify
    /// - `start_col`: the column-index of the first amplitude in `qureg` to
    ///   modify
    /// - `reals`: array of the real components of the new amplitudes
    /// - `imags`: array of the imaginary components of the new amplitudes
    ///
    /// # Errors
    ///
    /// - [`ArrayLengthError`]
    ///   - if `reals.len()` and `imags.len()` are different
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if `qureg` is not a density-matrix (i.e. is a state vector)
    ///   - if `start_row` is outside [0, 1 << [`qureg.num_qubits()`]]
    ///   - if `start_col` is outside [0, 1 << [`qureg.num_qubits()`]]
    ///   - if `reals.len()` is outside [0, `qureg.num_amps_total()`]
    ///   - if `reals.len()` is larger than the remaining number of amplitudes
    ///     from (`start_row`, `start_col`), column-wise
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let re = &[1., 2., 3.];
    /// let im = &[4., 5., 6.];
    /// let start_row = 1;
    /// let start_col = 1;
    /// qureg.set_density_amps(start_row, start_col, re, im);
    ///
    /// let amp = qureg.get_density_amp(2, 1).unwrap();
    ///
    /// assert!((amp.re - 2.).abs() < EPSILON);
    /// assert!((amp.im - 5.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`set_amps()`]: crate::Qureg::set_amps()
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`ArrayLengthError`]: crate::QuestError::ArrayLengthError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn set_density_amps(
        &mut self,
        start_row: i64,
        start_col: i64,
        reals: &[Qreal],
        imags: &[Qreal],
    ) -> Result<(), QuestError> {
        if reals.len() != imags.len() {
            return Err(QuestError::ArrayLengthError);
        }
        let num_amps = reals.len() as i64;
        catch_quest_exception(|| unsafe {
            ffi::setDensityAmps(
                self.reg,
                start_row,
                start_col,
                reals.as_ptr(),
                imags.as_ptr(),
                num_amps,
            );
        })
    }

    /// Shift the phase of a single qubit by a given angle.
    ///
    /// This is equivalent to a Z-axis rotation of the Bloch-sphere up to a
    /// global phase factor.
    ///
    /// For angle `theta`, this effects single-qubit unitary
    ///
    /// ```text
    ///   [ 1      0        ]
    ///   [ 0  exp(i theta) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to undergo a phase shift
    /// - `angle`: amount by which to shift the phase in radians
    ///
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if
    ///   - `target_qubit` is outside `[0, N)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let target_qubit = 1;
    /// let angle = 0.5;
    ///
    /// qureg.phase_shift(target_qubit, angle).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn phase_shift(
        &mut self,
        target_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::phaseShift(self.reg, target_qubit, angle);
        })
    }

    /// Controlled shift of the phase of a single qubit by a given angle.
    ///
    /// Introduce a phase factor `exp(i theta)` on state `|11>` of qubits
    /// `id_qubit1` and `id_qubit2`.  For angle `theta`, this effects
    /// the unitary
    ///
    /// ```text
    ///  [ 1  0  0        0      ]
    ///  [ 0  1  0        0      ]
    ///  [ 0  0  1        0      ]
    ///  [ 0  0  0  exp(i theta) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `id_qubit1`: first qubit in the state to phase shift
    /// - `id_qubit2`: second qubit in the state to phase shift
    /// - `angle`: amount by which to shift the phase in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if
    ///   - if `id_qubit1` or `id_qubit2` are outside `[0, N)`
    ///   - if `id_qubit1` and `id_qubit2` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let id_qubit1 = 0;
    /// let id_qubit2 = 2;
    /// let angle = 0.5;
    /// qureg
    ///     .controlled_phase_shift(id_qubit1, id_qubit2, angle)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_phase_shift(
        &mut self,
        id_qubit1: i32,
        id_qubit2: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledPhaseShift(self.reg, id_qubit1, id_qubit2, angle);
        })
    }

    /// Introduce a phase factor of the passed qubits.
    ///
    /// The phase factor is `exp(i theta)`, controlled by the state `|1..1>` of
    /// the passed qubits.
    ///
    /// # Parameters
    ///
    /// - `control_qubits`: array of qubits to phase shift
    /// - `angle`: amount by which to shift the phase in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if
    ///   - if `control_qubits.len()` is outside `[0, N)`
    ///   - if any qubit index in `control_qubits` is outside `[0, N)`
    ///   - if qubits in `control_qubits` are not unique
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubits = &[0, 1, 3];
    /// let angle = 0.5;
    /// qureg
    ///     .multi_controlled_phase_shift(control_qubits, angle)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_phase_shift(
        &mut self,
        control_qubits: &[i32],
        angle: Qreal,
    ) -> Result<(), QuestError> {
        let num_control_qubits = control_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledPhaseShift(
                self.reg,
                control_qubits.as_ptr(),
                num_control_qubits,
                angle,
            );
        })
    }

    /// Apply the (two-qubit) controlled phase flip gate.
    ///
    /// Also known as the controlled pauliZ gate. For each state, if both input
    /// qubits have value one, multiply the amplitude of that state by `-1`.
    /// This applies the two-qubit unitary:
    ///
    /// ```text
    ///   [ 1  0  0   0 ]
    ///   [ 0  1  0   0 ]
    ///   [ 0  0  1   0 ]
    ///   [ 0  0  0  -1 ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `id_qubit1`: first qubit in the state to operate on
    /// - `id_qubit2`: second qubit in the state to operate on
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if
    ///   - if `id_qubit1` or `id_qubit2` are outside `[0, N)`
    ///   - if `id_qubit1` and `id_qubit2` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    ///
    /// qureg.controlled_phase_flip(0, 1);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_phase_flip(
        &mut self,
        id_qubit1: i32,
        id_qubit2: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledPhaseFlip(self.reg, id_qubit1, id_qubit2);
        })
    }

    /// Apply the (multiple-qubit) controlled phase flip gate.
    ///
    /// Also known as the multiple-qubit controlled pauliZ gate. For each state,
    /// if all control qubits have value one, multiply the amplitude of that
    /// state by `-1`. This applies the many-qubit unitary:
    ///
    /// ```text
    ///   [  1  0  0  0   0  ]
    ///   [  0  1  0  0   0  ]
    ///   [  0  0   ...   0  ]
    ///   [  0  0  0  1   0  ]
    ///   [  0  0  0  0  -1  ]
    /// ```
    /// on the control qubits.
    ///
    /// # Parameters
    ///
    /// - `control_qubits`: array of input qubits
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if
    ///   - if `control_qubits.len()` is outside `[0, N)`
    ///   - if any qubit index in `control_qubits` is outside `[0, N)`
    ///   - if qubits in `control_qubits` are not unique
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    ///
    /// let control_qubits = &[0, 1, 3];
    /// qureg.multi_controlled_phase_flip(control_qubits);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_phase_flip(
        &mut self,
        control_qubits: &[i32],
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::multiControlledPhaseFlip(
                self.reg,
                control_qubits.as_ptr(),
                control_qubits.len() as i32,
            );
        })
    }

    /// Apply the single-qubit S gate.
    ///
    /// This is a rotation of `PI/2` around the Z-axis on the Bloch sphere, or
    /// the unitary:
    ///
    /// ```text
    ///   [ 1  0 ]
    ///   [ 0  i ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to operate upon
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    /// qureg.pauli_x(0).unwrap();
    ///
    /// qureg.s_gate(0).unwrap();
    ///
    /// let amp = qureg.get_imag_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn s_gate(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::sGate(self.reg, target_qubit);
        })
    }

    /// Apply the single-qubit T gate.
    ///
    /// This is a rotation of `PI/4` around the Z-axis on the Bloch sphere, or
    /// the unitary:
    ///
    /// ```text
    ///   [ 1       0       ]
    ///   [ 0  e^(i PI / 4) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to operate upon
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    /// qureg.pauli_x(0).unwrap();
    ///
    /// qureg.t_gate(0).unwrap();
    ///
    /// let amp = qureg.get_imag_amp(1).unwrap();
    /// assert!((amp - SQRT_2 / 2.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn t_gate(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::tGate(self.reg, target_qubit);
        })
    }

    /// Overwrite the amplitudes of `target_qureg` with those from `copy_qureg`.
    ///
    /// # Parameters
    ///
    /// - `copy_qureg`: the `Qureg` to have its quantum state clone into `self`
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `self` is a state-vector while `copy_qureg` is a density matrix
    ///     (and vice versa)
    ///   - if `self` and `copy_qureg` have different dimensions
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// let copy_qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.clone_qureg(&copy_qureg);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn clone_qureg(
        &mut self,
        copy_qureg: &Qureg<'_>,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::cloneQureg(self.reg, copy_qureg.reg);
        })
    }

    /// Performs a logical AND on all successCodes held by all processes.
    ///
    /// If any one process has a zero `success_code`, all processes will return
    /// a zero success code.
    ///
    /// # Parameters
    ///
    /// - `success_code`: `1` if process task succeeded, `0` if process task
    ///   failed
    ///
    /// # Returns
    ///
    /// `1` if all processes succeeded, `0` if any one process failed
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub fn sync_quest_success(success_code: i32) -> i32 {
        catch_quest_exception(|| unsafe { ffi::syncQuESTSuccess(success_code) })
            .expect("sync_quest_success should always succeed")
    }

    /// Copy the state-vector (or density matrix) into GPU memory.
    ///
    /// In GPU mode, this copies the state-vector (or density matrix) from RAM
    /// to VRAM / GPU-memory, which is the version operated upon by other calls
    /// to the API.
    ///
    /// In CPU mode, this function has no effect.
    ///
    /// In conjunction with [`copy_state_from_gpu()`][api-copy-state-from-gpu]
    /// (which should be called first), this allows a user to directly modify
    /// the state-vector in a hardware agnostic way. Note though that users
    /// should instead use [`set_amps()`][api-set-amps] if possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.copy_state_to_gpu();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-copy-state-from-gpu]: crate::Qureg::copy_state_from_gpu()
    /// [api-set-amps]: crate::Qureg::set_amps()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn copy_state_to_gpu(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::copyStateToGPU(self.reg);
        })
        .expect("copy_state_to_gpu should always succeed");
    }

    /// Copy the state-vector (or density matrix) from GPU memory.
    ///
    /// In GPU mode, this copies the state-vector (or density matrix) from GPU
    /// memory to RAM , where it can be accessed/modified  by the user.
    ///
    /// In CPU mode, this function has no effect.
    ///
    /// In conjunction with [`copy_state_to_gpu()`][api-copy-state-to-gpu] ,
    /// this allows a user to directly modify the state-vector in a hardware
    /// agnostic way. Note though that users should instead use
    /// [`set_amps()`][api-set-amps] if possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.copy_state_from_gpu();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-copy-state-to-gpu]: crate::Qureg::copy_state_to_gpu()
    /// [api-set-amps]: crate::Qureg::set_amps()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn copy_state_from_gpu(&mut self) {
        catch_quest_exception(|| unsafe { ffi::copyStateFromGPU(self.reg) })
            .expect("copy_state_from_gpu should always succeed");
    }

    /// Copy a part the state-vector (or density matrix) into GPU memory.
    ///
    /// In GPU mode, this copies a substate of the state-vector (or density
    /// matrix) from RAM to VRAM / GPU-memory.
    ///
    /// In CPU mode, this function has no effect.
    ///
    /// In conjunction with
    /// [`copy_substate_from_gpu()`][api-copy-substate-from-gpu], this allows a
    /// user to directly modify a subset of the amplitudes the state-vector
    /// in a hardware agnostic way, without having to load the entire state
    /// via [`copy_state_to_gpu()`][api-copy-state-to-gpu].
    ///
    /// Note though that users should instead use [`set_amps()`][api-set-amps]
    /// if possible.
    ///
    /// # Parameters
    ///
    /// - `start_ind` the index of the first amplitude to copy
    /// - `num_amps` the number of contiguous amplitudes to copy (starting with
    ///   `start_ind`)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `start_ind` is an invalid amplitude index
    ///   - if `num_amps` is greater than the remaining amplitudes in the state,
    ///     from `start_ind`
    ///
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-copy-substate-from-gpu]: crate::Qureg::copy_substate_from_gpu()
    /// [api-copy-state-to-gpu]: crate::Qureg::copy_state_to_gpu()
    /// [api-set-amps]: crate::Qureg::set_amps()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn copy_substate_to_gpu(
        &mut self,
        start_ind: i64,
        num_amps: i64,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::copySubstateToGPU(self.reg, start_ind, num_amps);
        })
    }

    /// Copy a part the state-vector (or density matrix) from GPU memory.
    ///
    /// In GPU mode, this copies a substate of the state-vector (or density
    /// matrix) from  to VRAM / GPU-memory to RAM, which is the version
    /// operated upon by other calls to the API.
    ///
    /// In CPU mode, this function has no effect.
    ///
    /// In conjunction with
    /// [`copy_substate_to_gpu()`][api-copy-substate-to-gpu], this allows a user
    /// to directly modify a subset of the amplitudes the state-vector in a
    /// hardware agnostic way, without having to load the entire state via
    /// [`copy_state_from_gpu()`][api-copy-state-from-gpu].
    ///
    /// Note though that users should instead use [`set_amps()`][api-set-amps]
    /// if possible.
    ///
    /// # Parameters
    ///
    /// - `start_ind` the index of the first amplitude to copy
    /// - `num_amps` the number of contiguous amplitudes to copy (starting with
    ///   `start_ind`)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `start_ind` is an invalid amplitude index
    ///   - if `num_amps` is greater than the remaining amplitudes in the state,
    ///     from `start_ind`
    ///
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-copy-substate-to-gpu]: crate::Qureg::copy_substate_to_gpu()
    /// [api-copy-state-from-gpu]: crate::Qureg::copy_state_from_gpu()
    /// [api-set-amps]: crate::Qureg::set_amps()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn copy_substate_from_gpu(
        &mut self,
        start_ind: i64,
        num_amps: i64,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::copySubstateToGPU(self.reg, start_ind, num_amps);
        })
    }

    /// Get the complex amplitude at a given index in the state vector.
    ///
    /// # Parameters
    ///
    /// - `index`: index in state vector of probability amplitudes
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `self` is a density matrix
    ///   - if `index` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.get_amp(0).unwrap().re;
    /// assert!((amp - 0.5).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_amp(
        &self,
        index: i64,
    ) -> Result<Qcomplex, QuestError> {
        catch_quest_exception(|| unsafe { ffi::getAmp(self.reg, index) })
            .map(Into::into)
    }

    /// Get the real part of the probability amplitude at an index in
    /// the state vector.
    ///
    /// # Parameters
    ///
    /// - `index`: index in state vector of probability amplitudes
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is a density matrix
    ///   - if `index` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.get_real_amp(0).unwrap();
    /// assert!((amp - 0.5).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_real_amp(
        &self,
        index: i64,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe { ffi::getRealAmp(self.reg, index) })
    }

    /// Get the imaginary part of the probability amplitude at an index
    /// in the state vector.
    ///
    /// # Parameters
    ///
    /// - `index`: index in state vector of probability amplitudes
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is a density matrix
    ///   - if `index` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.get_imag_amp(0).unwrap();
    /// assert!(amp.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_imag_amp(
        &self,
        index: i64,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe { ffi::getImagAmp(self.reg, index) })
    }

    /// Get the probability of a state-vector at an index in the full state
    /// vector.
    ///
    /// # Parameters
    ///
    /// - `index`: index in state vector of probability amplitudes
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is a density matrix
    ///   - if `index` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.get_prob_amp(0).unwrap();
    /// assert!((amp - 0.25).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_prob_amp(
        &self,
        index: i64,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe { ffi::getProbAmp(self.reg, index) })
    }

    /// Get an amplitude from a density matrix at a given row and column.
    ///
    /// # Parameters
    ///
    /// - `row`: row of the desired amplitude in the density matrix
    /// - `col`: column of the desired amplitude in the density matrix
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is a state vector
    ///   - if `row` or `col` are outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.get_density_amp(0, 0).unwrap().re;
    /// assert!((amp - 0.25).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_density_amp(
        &self,
        row: i64,
        col: i64,
    ) -> Result<Qcomplex, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::getDensityAmp(self.reg, row, col)
        })
        .map(Into::into)
    }

    /// A debugging function which calculates the total probability of the
    /// qubits.
    ///
    /// This function should always be 1 for correctly normalized states
    /// (hence returning a real number).
    /// For state-vectors, this is the norm of the entire state-vector
    /// (the sum of the absolute-value-squared of every amplitude).
    /// For density matrices, it is the trace.
    /// For un-normalized density matrices (those directly modified or
    /// initialized by the user),  this function returns the real component
    /// of the trace.
    ///
    /// Note this calculation utilizes Kahan summation for greater accuracy,
    /// and hence is not parallelized and so will be slower than other
    /// functions.
    ///
    /// # Returns
    ///
    /// The total probability of the qubits in this `Qureg` being in any state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let amp = qureg.calc_total_prob();
    /// assert!((amp - 1.).abs() < EPSILON)
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub fn calc_total_prob(&self) -> Qreal {
        catch_quest_exception(|| unsafe { ffi::calcTotalProb(self.reg) })
            .expect("calc_total_prop should always succeed")
    }

    /// Apply a single-qubit unitary parameterized by two given complex scalars.
    ///
    /// Given valid complex numbers `alpha` and `beta`, applies the unitary:
    ///
    /// ```text
    /// [ alpha -beta.conj() ]
    /// [ beta  alpha.conj() ]
    /// ```
    ///
    /// Valid `alpha`, `beta` satisfy `|alpha|^2 + |beta|^2 = 1`.
    /// The target unitary is general up to a global phase factor.
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to operate on
    /// - `alpha`: complex unitary parameter (row 1, column 1)
    /// - `beta`: complex unitary parameter (row 2, column 1)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///   - if  `alpha`, `beta` don't satisfy: `|alpha|^2 + |beta|^2 = 1`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    ///
    /// let norm = SQRT_2.recip();
    /// let alpha = Qcomplex::new(0., norm);
    /// let beta = Qcomplex::new(0., norm);
    /// qureg.compact_unitary(0, alpha, beta).unwrap();
    ///
    /// let other_qureg = {
    ///     let mut other_qureg =
    ///         Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///     other_qureg.init_zero_state();
    ///     other_qureg.hadamard(0).unwrap();
    ///     other_qureg
    /// };
    ///
    /// let fidelity = qureg.calc_fidelity(&other_qureg).unwrap();
    /// assert!((fidelity - 1.).abs() < 10. * EPSILON,);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn compact_unitary(
        &mut self,
        target_qubit: i32,
        alpha: Qcomplex,
        beta: Qcomplex,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::compactUnitary(
                self.reg,
                target_qubit,
                alpha.into(),
                beta.into(),
            );
        })
    }

    /// Apply a general single-qubit unitary (including a global phase factor).
    ///
    /// The passed 2x2 `ComplexMatrix` must be unitary, otherwise an error is
    /// thrown.
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to operate on
    /// - `u`: single-qubit unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///   - if `u` is not unitary
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_zero_state();
    ///
    /// let norm = SQRT_2.recip();
    /// let mtr = ComplexMatrix2::new(
    ///     [[norm, norm], [norm, -norm]],
    ///     [[0., 0.], [0., 0.]],
    /// );
    /// qureg.unitary(0, &mtr).unwrap();
    ///
    /// let other_qureg = {
    ///     let mut other_qureg =
    ///         Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///     other_qureg.hadamard(0).unwrap();
    ///     other_qureg
    /// };
    /// let fidelity = qureg.calc_fidelity(&other_qureg).unwrap();
    /// assert!((fidelity - 1.).abs() < 10. * EPSILON,);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn unitary(
        &mut self,
        target_qubit: i32,
        u: &ComplexMatrix2,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::unitary(self.reg, target_qubit, u.0);
        })
    }

    /// Rotate a single qubit by a given angle around the X-axis of the
    /// Bloch-sphere.
    ///
    /// For angle `theta`, this applies
    /// ```text
    /// [    cos(theta/2)   -i sin(theta/2) ]
    /// [ -i sin(theta/2)      cos(theta/2) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `rot_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `rot_qubit` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// let theta = PI;
    ///
    /// qureg.rotate_x(0, theta).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn rotate_x(
        &mut self,
        rot_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::rotateX(self.reg, rot_qubit, angle);
        })
    }

    /// Rotate a single qubit by a given angle around the Y-axis of the
    /// Bloch-sphere.
    ///
    /// For angle `theta`, this applies
    /// ```text
    /// [  cos(theta/2)   -sin(theta/2) ]
    /// [ -sin(theta/2)    cos(theta/2) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `rot_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `rot_qubit` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// let theta = PI;
    ///
    /// qureg.rotate_y(0, theta).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn rotate_y(
        &mut self,
        rot_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::rotateY(self.reg, rot_qubit, angle);
        })
    }

    /// Rotate a single qubit by a given angle around the Z-axis of the
    /// Bloch-sphere.
    ///
    /// For angle `theta`, this applies
    /// ```text
    /// [ exp(-i theta/2)         0     ]
    /// [       0          exp(theta/2) ]
    /// ```
    ///
    /// # Parameters
    ///
    /// - `rot_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `rot_qubit` is outside [0, [`num_qubits()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// let theta = PI;
    ///
    /// qureg.rotate_z(0, theta).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn rotate_z(
        &mut self,
        rot_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::rotateZ(self.reg, rot_qubit, angle);
        })
    }

    /// Rotate a single qubit by a given angle around a given axis.
    ///
    /// The axis of rotation is given by a [`Vector`] on the Bloch-sphere.      
    /// The vector must not be zero (or else an error is thrown), but needn't be
    /// unit magnitude, since the normalization will be computed by by `QuEST`.
    ///
    /// # Parameters
    ///
    /// - `rot_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate in radians
    /// - `axis`: vector around which to rotate (can be non-unit; will be
    ///   normalized)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if `rot_qubit` is outside [0, [`num_qubits()`])
    ///   - if `axis` is the zero vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let angle = 2.0 * PI;
    /// let axis = &Vector::new(0., 0., 1.);
    /// qureg.rotate_around_axis(0, angle, axis).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`Vector`]: crate::Vector
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn rotate_around_axis(
        &mut self,
        rot_qubit: i32,
        angle: Qreal,
        axis: &Vector,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::rotateAroundAxis(self.reg, rot_qubit, angle, axis.0);
        })
    }

    /// Applies a controlled rotation by a given angle around the X-axis of the
    /// Bloch-sphere.
    ///
    /// The target qubit is rotated in states where the control qubit has value
    /// `1`.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: qubit which has value `1` in the rotated states
    /// - `target_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate the target qubit in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if either `control_qubit` or `target_qubit` are outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubit = 1;
    /// let target_qubit = 0;
    /// let angle = PI;
    /// qureg
    ///     .controlled_rotate_x(control_qubit, target_qubit, angle)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_rotate_x(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledRotateX(
                self.reg,
                control_qubit,
                target_qubit,
                angle,
            );
        })
    }

    /// Applies a controlled rotation by a given angle around the Y-axis of the
    /// Bloch-sphere.
    ///
    /// The target qubit is rotated in states where the control qubit has value
    /// `1`.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: qubit which has value `1` in the rotated states
    /// - `target_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate the target qubit in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if either `control_qubit` or `target_qubit` are outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubit = 1;
    /// let target_qubit = 0;
    /// let angle = PI;
    /// qureg
    ///     .controlled_rotate_y(control_qubit, target_qubit, angle)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_rotate_y(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledRotateY(
                self.reg,
                control_qubit,
                target_qubit,
                angle,
            );
        })
    }

    /// Applies a controlled rotation by a given angle around the Z-axis of the
    /// Bloch-sphere.
    ///
    /// The target qubit is rotated in states where the control qubit has value
    /// `1`.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: qubit which has value `1` in the rotated states
    /// - `target_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate the target qubit in radians
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if either `control_qubit` or `target_qubit` are outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubit = 1;
    /// let target_qubit = 0;
    /// let angle = PI;
    /// qureg
    ///     .controlled_rotate_z(control_qubit, target_qubit, angle)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_rotate_z(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledRotateZ(
                self.reg,
                control_qubit,
                target_qubit,
                angle,
            );
        })
    }

    /// Applies a controlled rotation by  around a given vector of the
    /// Bloch-sphere.
    ///
    /// The vector must not be zero (else an error is thrown), but needn't be
    /// unit magnitude.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: qubit which has value `1` in the rotated states
    /// - `target_qubit`: qubit to rotate
    /// - `angle`: angle by which to rotate in radians
    /// - `axis`: vector around which to rotate (can be non-unit; will be
    ///   normalized)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if either `control_qubit` or `target_qubit` are outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///   - if `axis` is the zero vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubit = 1;
    /// let target_qubit = 0;
    /// let angle = PI;
    /// let vector = Vector::new(0., 0., 1.);
    /// qureg
    ///     .controlled_rotate_around_axis(
    ///         control_qubit,
    ///         target_qubit,
    ///         angle,
    ///         &vector,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`Vector`]: crate::Vector
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_rotate_around_axis(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        angle: Qreal,
        axis: &Vector,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledRotateAroundAxis(
                self.reg,
                control_qubit,
                target_qubit,
                angle,
                axis.0,
            );
        })
    }

    /// Apply a controlled unitary parameterized by
    /// two given complex scalars.
    ///
    ///  Given valid complex numbers `alpha` and `beta`, applies the two-qubit
    /// unitary:
    ///
    /// ```text
    /// [ alpha -beta.conj() ]
    /// [ beta  alpha.conj() ]
    /// ```
    ///
    /// Valid `alpha`, `beta` satisfy `|alpha|^2 + |beta|^2 = 1`.
    /// The target unitary is general up to a global phase factor.  
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: applies unitary if this qubit is `1`
    /// - `target_qubit`: qubit to operate on
    /// - `alpha`: complex unitary parameter (row 1, column 1)
    /// - `beta`: complex unitary parameter (row 2, column 1)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `target_qubit` or `control_qubit` is outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if `control_qubits` and `target_qubit` are equal
    ///   - if  `alpha`, `beta` don't satisfy: `|alpha|^2 + |beta|^2 = 1`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let norm = SQRT_2.recip();
    /// let alpha = Qcomplex::new(0., norm);
    /// let beta = Qcomplex::new(0., norm);
    /// qureg.controlled_compact_unitary(0, 1, alpha, beta).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_compact_unitary(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        alpha: Qcomplex,
        beta: Qcomplex,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledCompactUnitary(
                self.reg,
                control_qubit,
                target_qubit,
                alpha.into(),
                beta.into(),
            );
        })
    }

    /// Apply a general controlled unitary.
    ///
    /// The unitary can include a global phase factor and is applied
    /// to the target qubit if the control qubit has value `1`.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: applies unitary if this qubit is `1`
    /// - `target_qubit`: qubit to operate on
    /// - `u`: single-qubit unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `target_qubit` or `control_qubit` is outside [0,
    ///    [`qureg.num_qubits()`]).
    ///   - if `control_qubits` and `target_qubit` are equal
    ///   - if `u` is not unitary
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let norm = SQRT_2.recip();
    /// let mtr = &ComplexMatrix2::new(
    ///     [[norm, norm], [norm, -norm]],
    ///     [[0., 0.], [0., 0.]],
    /// );
    /// qureg.controlled_unitary(0, 1, mtr).unwrap();
    /// ```
    ///
    /// See [`QuEST` `other_qureg.API`]nformation.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_unitary(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
        u: &ComplexMatrix2,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledUnitary(self.reg, control_qubit, target_qubit, u.0);
        })
    }

    /// Apply a general multiple-control single-target unitary.
    ///
    /// The unitary can include a global phase factor. Any number of control
    /// qubits can be specified, and if all have value `1`, the given
    /// unitary is applied to the target qubit.
    ///
    /// # Parameters
    ///
    /// - `control_qubits`: applies unitary if all qubits in this slice are
    ///   equal to `1`
    /// - `target_qubit`: qubit to operate on
    /// - `u`: single-qubit unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit` or any of `control_qubits` is outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if any qubit in `control_qubits` is repeated
    ///   - if `control_qubits` contains `target_qubit`
    ///   - if `u` is not unitary
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let norm = SQRT_2.recip();
    /// let mtr = &ComplexMatrix2::new(
    ///     [[norm, norm], [norm, -norm]],
    ///     [[0., 0.], [0., 0.]],
    /// );
    /// qureg.multi_controlled_unitary(&[1, 2], 0, mtr).unwrap();
    /// ```
    ///
    /// See [QuEST API] `fother_qureg.or` ation.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_unitary(
        &mut self,
        control_qubits: &[i32],
        target_qubit: i32,
        u: &ComplexMatrix2,
    ) -> Result<(), QuestError> {
        let num_control_qubits = control_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledUnitary(
                self.reg,
                control_qubits.as_ptr(),
                num_control_qubits,
                target_qubit,
                u.0,
            );
        })
    }

    /// Apply the single-qubit Pauli-X gate.
    ///
    /// # Parameters
    ///
    ///  - `target_qubit`: qubit to operate on
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`qureg.num_qubits()`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.pauli_x(0).unwrap();
    ///
    /// let amp = qureg.get_real_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn pauli_x(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::pauliX(self.reg, target_qubit);
        })
    }

    /// Apply the single-qubit Pauli-Y gate.
    ///
    /// # Parameters
    ///
    ///  - `target_qubit`: qubit to operate on
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`qureg.num_qubits()`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.pauli_y(0).unwrap();
    ///
    /// let amp = qureg.get_imag_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn pauli_y(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::pauliY(self.reg, target_qubit);
        })
    }

    /// Apply the single-qubit Pauli-Z gate.
    ///
    /// # Parameters
    ///
    ///  - `target_qubit`: qubit to operate on
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`qureg.num_qubits()`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.pauli_z(0).unwrap();
    ///
    /// let amp = qureg.get_real_amp(0).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn pauli_z(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::pauliZ(self.reg, target_qubit);
        })
    }

    /// Apply the single-qubit Hadamard gate.
    ///
    /// This function applies the following unitary on `qubit`:
    ///
    /// ```text
    /// SQRT_2.recip() *
    ///     [ 1  1 ]
    ///     [ 1 -1 ]
    /// ```
    ///
    /// # Parameters
    ///
    ///  - `target_qubit`: qubit to operate on
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`qureg.num_qubits()`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.hadamard(0).unwrap();
    ///
    /// let amp = qureg.get_real_amp(0).unwrap();
    /// assert!((amp - SQRT_2.recip()).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn hadamard(
        &mut self,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::hadamard(self.reg, target_qubit);
        })
    }

    /// Apply the controlled not (single control, single target) gate.
    ///
    /// The gate is also known as the c-X, c-sigma-X, c-Pauli-X and c-bit-flip
    /// gate. This applies pauliX to the target qubit if the control qubit
    /// has value 1. This effects the two-qubit unitary:
    ///
    /// ```text
    ///  [ 1  0  0  0 ]
    ///  [ 0  1  0  0 ]
    ///  [ 0  0  0  1 ]
    ///  [ 0  0  1  0 ]
    /// ```
    ///
    /// on the control and target qubits.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: "nots" the target if this qubit is `1`
    /// - `target_qubit`: qubit to "not"
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// qureg.controlled_not(1, 0).unwrap();
    ///
    /// let amp = qureg.get_real_amp(3).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_not(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledNot(self.reg, control_qubit, target_qubit);
        })
    }

    /// Apply a NOT (or Pauli X) gate with multiple control and target qubits.
    ///
    /// This applies pauliX to qubits `targs` on every basis state for which the
    /// control qubits `ctrls` are all in the `|1>` state. The ordering within
    /// each of `ctrls` and `targs` has no effect on the operation.
    ///
    /// This function is equivalent, but significantly faster (approximately
    /// `targs.len()` times) than applying controlled NOTs on each qubit in
    /// `targs` in turn.
    ///
    /// In distributed mode, this operation requires at most a single round of)
    /// pair-wise communication between nodes, and hence is as efficient as
    /// [`pauli_x()`][api-pauli-x].
    ///
    /// # Parameters
    ///
    ///  - `ctrls`: a list of the control qubit indices
    ///  - `targs`: a list of the qubits to be targeted by the X gates
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit in `ctrls` and `targs` is invalid, i.e. outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if the length of `targs` or `ctrls` is larger than
    ///     [`qureg.num_qubits()`]
    ///   - if `ctrls` or `targs` contain any repetitions
    ///   - if any qubit in `ctrls` is also in `targs` (and vice versa)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let ctrls = &[0, 1];
    /// let targs = &[2, 3];
    /// qureg
    ///     .multi_controlled_multi_qubit_not(ctrls, targs)
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-pauli-x]: crate::Qureg::pauli_x()
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_multi_qubit_not(
        &mut self,
        ctrls: &[i32],
        targs: &[i32],
    ) -> Result<(), QuestError> {
        let num_ctrls = ctrls.len() as i32;
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledMultiQubitNot(
                self.reg,
                ctrls.as_ptr(),
                num_ctrls,
                targs.as_ptr(),
                num_targs,
            );
        })
    }

    /// Apply a NOT (or Pauli X) gate with multiple target qubits.
    ///
    /// This has the same  effect as (but is much faster than) applying each
    /// single-qubit NOT gate in turn.
    ///
    /// The ordering within `targs` has no effect on the operation.
    ///
    /// This function is equivalent, but significantly faster (approximately
    /// `targs.len()` times) than applying NOT on each qubit in `targs` in turn.
    ///
    /// In distributed mode, this operation requires at most a single round of)
    /// pair-wise communication between nodes, and hence is as efficient as
    /// [`pauli_x()`][api-pauli-x].
    ///
    /// # Parameters
    ///
    ///  - `targs`: a list of the qubits to be targeted by the X gates
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit in `targs` is invalid, i.e. outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if the length of `targs` is larger than [`qureg.num_qubits()`]
    ///   - if `targs` contains any repetitions
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let targs = &[0, 1];
    /// qureg.multi_qubit_not(targs).unwrap();
    ///
    /// let amp = qureg.get_real_amp(3).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-pauli-x]: crate::Qureg::pauli_x()
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_qubit_not(
        &mut self,
        targs: &[i32],
    ) -> Result<(), QuestError> {
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            let targs_ptr = targs.as_ptr();
            ffi::multiQubitNot(self.reg, targs_ptr, num_targs);
        })
    }

    /// Apply the controlled pauli Y (single control, single target) gate.
    ///
    /// The gate is also known as the c-Y and c-sigma-Y gate.
    /// This applies pauli Y to the target qubit, if the control qubit has value
    /// 1. This effects the two-qubit unitary:
    ///
    /// ```text
    ///  [ 1  0  0   0 ]
    ///  [ 0  1  0   0 ]
    ///  [ 0  0  0  -i ]
    ///  [ 0  0  i   0 ]
    /// ```
    ///
    /// on the control and target qubits.
    ///
    /// # Parameters
    ///
    /// - `control_qubit`: applies pauli Y the target if this qubit is `1`
    /// - `target_qubit`: qubit to modify
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `control_qubit` or `target_qubit` is outside [0,
    ///     [`num_qubits()`])
    ///   - if `control_qubit` and `target_qubit` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// qureg.controlled_pauli_y(1, 0).unwrap();
    ///
    /// let amp = qureg.get_imag_amp(3).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_pauli_y(
        &mut self,
        control_qubit: i32,
        target_qubit: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledPauliY(self.reg, control_qubit, target_qubit);
        })
    }

    /// Gives the probability of a qubit being measured in the given outcome.
    ///
    /// This performs no actual measurement and does not change the state of the
    /// qubits.
    ///
    /// - For state-vectors, this function works by summing the
    ///   absolute-value-squared of every amplitude in the state-vector for
    ///   which `measure_qubit = 0`. If `outcome = 1`, it returns `1` minus this
    ///   value. Hence for unnormalized state-vectors, this result will differ
    ///   from the absolute-value-squared of every amplitude where
    ///   `measure_qubit = outcome`.
    ///
    /// - For density matrices, this function sums the diagonal values (should
    ///   be real) corresponding to `measure_qubit = 0` (returning 1 minus this
    ///   if `outcome = 1`).
    ///
    /// # Parameters
    ///
    /// - `measure_qubit`: qubit to study
    /// - `outcome`: for which to find the probability of the qubit being
    ///   measured in
    ///
    /// # Returns
    ///
    /// Returns probability of qubit `measure_qubit` being measured in the given
    /// outcome.
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `measure_qubit` is outside [0, [`num_qubits()`])
    ///   - if `outcome` is not in {0, 1}
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let prob = qureg.calc_prob_of_outcome(0, 0).unwrap();
    /// assert!((prob - 1.).abs() < EPSILON);
    /// let prob = qureg.calc_prob_of_outcome(0, 1).unwrap();
    /// assert!(prob.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn calc_prob_of_outcome(
        &self,
        measure_qubit: i32,
        outcome: i32,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::calcProbOfOutcome(self.reg, measure_qubit, outcome)
        })
    }

    /// Calculate probabilities of every outcome of the sub-register.
    ///
    /// This function populates `outcome_probs` with the probabilities of every
    /// outcome of the sub-register contained in `qubits`.
    ///
    /// This performs no actual measurement and does not modify `qureg`.
    ///
    /// - `outcome_probs` must be a pre-allocated array of length
    ///   `2^qubits.len()`. In distributed mode, every node receives the full
    ///   list of outcome probabilities.
    ///
    /// - Note that the indices in `qubits` need not be adjacent nor ordered.
    ///   The order of `qubits` determines the order of `outcome_probs`, whereby
    ///   `qubits` are treated as *increasing* significance.
    ///
    /// - Since all probability amplitudes of a state-vector are ultimately
    ///   involved in the output probabilities, this function works as expected
    ///   for unnormalized states. This is similarly true for density matrices,
    ///   where all  diagonal elements are involved, although only the real
    ///   values of the diagonal elements will be consulted.
    ///
    /// # Parameters
    ///
    /// - `outcome_probs`: a pre-allocated array of length `1 << n`, where `n =
    ///   qubits.len()`  which will be modified to contain all outcome
    ///   probabilities
    /// - `qubits`: a list of qubits to study
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any index in `qubits` is invalid, i.e. outside [0,
    ///     [`num_qubits()`])
    ///   - if `qubits` contains any repetitions
    /// - [`ArrayLengthError`],
    ///   - if `outcome_probs.len() < 1 << qubits.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let qubits = &[1, 2];
    /// let outcome_probs = &mut vec![0.; 4];
    /// qureg
    ///     .calc_prob_of_all_outcomes(outcome_probs, qubits)
    ///     .unwrap();
    /// assert_eq!(outcome_probs, &vec![1., 0., 0., 0.]);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`ArrayLengthError`]: crate::QuestError::ArrayLengthError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn calc_prob_of_all_outcomes(
        &self,
        outcome_probs: &mut [Qreal],
        qubits: &[i32],
    ) -> Result<(), QuestError> {
        if outcome_probs.len() < 1 << qubits.len() {
            return Err(QuestError::ArrayLengthError);
        }
        let num_qubits = qubits.len() as i32;
        let outcome_probs_ptr = outcome_probs.as_mut_ptr();
        catch_quest_exception(|| unsafe {
            ffi::calcProbOfAllOutcomes(
                outcome_probs_ptr,
                self.reg,
                qubits.as_ptr(),
                num_qubits,
            );
        })
    }

    /// Updates `qureg` to be consistent with measuring qubit in the given
    /// outcome.
    ///
    /// Returns the probability of such a measurement outcome. This is
    /// effectively performing a renormalizing projection, or a measurement
    /// with a forced outcome.  This is an irreversible change to the state,
    /// whereby computational states inconsistent with the outcome are
    /// given zero amplitude and the `Qureg` is renormalized.  The given
    /// outcome must not have a near zero probability, else it cannot
    /// be collapsed into.
    ///
    /// Note that the collapse probably used for renormalization is calculated
    /// for  `outcome = 0`, and assumed `1` minus this probability if
    /// `outcome = 1`.  Hence this routine will not correctly project
    /// un-normalised `Qureg`s onto  `outcome = 1`.
    ///
    /// To avoid renormalization after projection, or force projection into
    /// non-physical states with very small probability, use
    /// [`apply_projector()`].  
    ///
    /// # Parameters
    ///
    /// - `measure_qubit`: qubit to measure
    /// - `outcome`: to force the measure qubit to enter
    ///
    /// # Returns
    ///
    /// Probability of the (forced) measurement outcome.
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `measure_qubit` is outside `[0, N)`
    ///   - if `outcome` is not in `{0, 1}`
    ///   - if the probability of `outcome` is zero (within machine epsilon)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// qureg.collapse_to_outcome(0, 0).unwrap();
    ///
    /// // QuEST throws an exception if probability of outcome is 0.
    /// qureg.init_zero_state();
    /// qureg.collapse_to_outcome(0, 1).unwrap_err();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`apply_projector()`]: Qureg::apply_projector()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn collapse_to_outcome(
        &mut self,
        measure_qubit: i32,
        outcome: i32,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::collapseToOutcome(self.reg, measure_qubit, outcome)
        })
    }

    /// Measures a single qubit, collapsing it randomly to `0` or `1`.
    ///
    /// Outcome probabilities are weighted by the state vector, which is
    /// irreversibly  changed after collapse to be consistent with the
    /// outcome.
    ///
    /// The random number generator is seeded by [`seed_quest_default()`]
    /// within  [`QuestEnv::new()`], unless later overridden by
    /// [`seed_quest()`].
    ///
    /// # Parameters
    ///
    /// - `measure_qubit`: index of a qubit to measure
    ///
    /// # Returns
    ///
    /// The measurement outcome, `0` or `1`.
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `measure_qubit` is outside `[0, N)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// // Prepare an entangled state `|00> + |11>`
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    ///
    /// // Qubits are entangled now
    /// let outcome1 = qureg.measure(0).unwrap();
    /// let outcome2 = qureg.measure(1).unwrap();
    ///
    /// assert_eq!(outcome1, outcome2);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`seed_quest_default()`]: crate::seed_quest_default()
    /// [`QuestEnv::new()`]: QuestEnv::new()
    /// [`seed_quest()`]: crate::seed_quest()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn measure(
        &mut self,
        measure_qubit: i32,
    ) -> Result<i32, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::measure(self.reg, measure_qubit)
        })
    }

    /// Measures a single qubit, collapsing it randomly to 0 or 1
    ///
    /// Additionally, the function gives the probability of that outcome.
    /// Outcome probabilities are weighted by the state vector, which is
    /// irreversibly changed after collapse to be consistent with the outcome.
    ///
    /// The random number generator is seeded by [`seed_quest_default()`]
    /// within  [`QuestEnv::new()`], unless later overridden by
    /// [`seed_quest()`].
    ///
    /// # Parameters
    ///
    /// - `measure_qubit`: index of a qubit to measure
    /// - `outcome_prob`: a mutable reference to a `Qreal` which is set to the
    ///   probability of the occurred outcome
    ///
    /// # Returns
    ///
    /// The measurement outcome, `0` or `1`.
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `measure_qubit` is outside `[0, N)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// // Prepare an entangled state `|00> + |11>`
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    ///
    /// // Qubits are entangled now
    /// let prob = &mut -1.;
    /// let outcome1 = qureg.measure_with_stats(0, prob).unwrap();
    /// assert!((*prob - 0.5).abs() < EPSILON);
    ///
    /// let outcome2 = qureg.measure_with_stats(1, prob).unwrap();
    /// assert!((*prob - 1.).abs() < EPSILON);
    ///
    /// assert_eq!(outcome1, outcome2);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`seed_quest_default()`]: crate::seed_quest_default()
    /// [`QuestEnv::new()`]: QuestEnv::new()
    /// [`seed_quest()`]: crate::seed_quest()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn measure_with_stats(
        &mut self,
        measure_qubit: i32,
        outcome_prob: &mut Qreal,
    ) -> Result<i32, QuestError> {
        let outcome_prob_ptr = outcome_prob as *mut _;
        catch_quest_exception(|| unsafe {
            ffi::measureWithStats(self.reg, measure_qubit, outcome_prob_ptr)
        })
    }

    /// Enable QASM recording.
    ///
    /// Gates applied to qureg will here-after be added to a growing log of QASM
    /// instructions, progressively consuming more memory until disabled with
    /// [`stop_recording_qasm()`]. The QASM log is bound to this qureg instance.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.start_recording_qasm();
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    /// qureg.stop_recording_qasm();
    ///
    /// qureg.print_recorded_qasm();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`stop_recording_qasm()`]: Qureg::stop_recording_qasm()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn start_recording_qasm(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::startRecordingQASM(self.reg);
        })
        .expect("start_recording_qasm should always succeed");
    }

    /// Disable QASM recording.
    ///
    /// The recorded QASM will be maintained in qureg and continue to be
    /// appended to if [`start_recording_qasm()`] is recalled.
    ///
    /// Has no effect if this `Qureg` was not already recording operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.start_recording_qasm();
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    /// qureg.stop_recording_qasm();
    ///
    /// qureg.print_recorded_qasm();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`start_recording_qasm()`]: Qureg::start_recording_qasm()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn stop_recording_qasm(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::stopRecordingQASM(self.reg);
        })
        .expect("stop_recording_qasm should always succeed");
    }

    /// Clear all QASM so far recorded.
    ///
    /// This does not start or stop recording.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.start_recording_qasm();
    /// qureg.hadamard(0).unwrap();
    ///
    /// qureg.clear_recorded_qasm();
    ///
    /// qureg.controlled_not(0, 1).unwrap();
    /// qureg.stop_recording_qasm();
    /// qureg.print_recorded_qasm();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn clear_recorded_qasm(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::clearRecordedQASM(self.reg);
        })
        .expect("clear_recorded_qasm should always succeed");
    }

    /// Print recorded QASM to stdout.
    ///
    /// This does not clear the QASM log, nor does it start or stop QASM
    /// recording.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.start_recording_qasm();
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    /// qureg.stop_recording_qasm();
    ///
    /// qureg.print_recorded_qasm();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn print_recorded_qasm(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::printRecordedQASM(self.reg);
        })
        .expect("print_recorded_qasm should always succeed");
    }

    /// Writes recorded QASM to a file, throwing an error if inaccessible.
    ///
    /// # Parameters
    ///
    /// - `filename`: the filename of the file to contain the recorded QASM
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `filename` cannot be written to
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.start_recording_qasm();
    /// qureg.hadamard(0).and(qureg.controlled_not(0, 1)).unwrap();
    /// qureg.stop_recording_qasm();
    ///
    /// qureg.write_recorded_qasm_to_file("/dev/null").unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_recorded_qasm_to_file(
        &mut self,
        filename: &str,
    ) -> Result<(), QuestError> {
        unsafe {
            let filename_cstr =
                CString::new(filename).map_err(QuestError::NulError)?;
            catch_quest_exception(|| {
                ffi::writeRecordedQASMToFile(
                    self.reg,
                    (*filename_cstr).as_ptr(),
                );
            })
        }
    }

    /// Mixes a density matrix to induce single-qubit dephasing noise.
    ///
    /// With probability `prob`, applies Pauli Z to `target_qubit` in `qureg`.
    ///
    /// This transforms `qureg = rho` into the mixed state:
    ///
    /// ```text
    /// (1 - prob) * rho  +  prob * Z_q rho Z_q,
    /// ```
    ///
    /// where `q = target_qubit`. The coefficient `prob` cannot exceed `1/2`,
    /// which maximally mixes `target_qubit`.
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit upon which to induce dephasing noise
    /// - `prob`: the probability of the phase error occurring
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///   - if `prob` is not in `[0, 1/2]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// qureg.mix_dephasing(0, 0.5).unwrap();
    ///
    /// let amp = qureg.get_density_amp(0, 0).unwrap();
    /// assert!((amp.re - 0.25).abs() < EPSILON);
    /// let amp = qureg.get_density_amp(0, 1).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_dephasing(
        &mut self,
        target_qubit: i32,
        prob: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixDephasing(self.reg, target_qubit, prob);
        })
    }

    /// Mixes a density matrix `qureg` to induce two-qubit dephasing noise.
    ///
    /// With probability `prob`, applies Pauli Z to either or both in `qureg`.
    ///
    /// This transforms `qureg = rho` into the mixed state:
    ///
    /// ```text
    /// (1 - prob) * rho  +  prob * 1/3 * (
    ///         Z_a  rho  Z_a +
    ///         Z_b  rho  Z_b +
    ///         Z_a Z_b rho Z_a Z_b
    ///    )
    /// ```
    ///
    /// where `a = qubit1`, `b=qubit2`. The coefficient `prob` cannot exceed
    /// `3/4`, at which maximal mixing occurs.
    ///
    /// # Parameters
    ///
    /// - `qubit1`: qubit upon which to induce dephasing noise
    /// - `qubit2`: qubit upon which to induce dephasing noise
    /// - `prob`: the probability of the phase error occurring
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `qubit1` or `qubit2` are outside [0, [`num_qubits()`]).
    ///   - if `qubit1 = qubit2`
    ///   - if `prob` is not in `[0, 3/4]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// qureg.mix_two_qubit_dephasing(0, 1, 0.75).unwrap();
    ///
    /// let amp = qureg.get_density_amp(0, 0).unwrap();
    /// assert!((amp.re - 0.125).abs() < EPSILON);
    /// let amp = qureg.get_density_amp(0, 1).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_two_qubit_dephasing(
        &mut self,
        qubit1: i32,
        qubit2: i32,
        prob: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixTwoQubitDephasing(self.reg, qubit1, qubit2, prob);
        })
    }

    /// Mixes a density matrix to induce single-qubit homogeneous
    /// depolarising noise.
    /// This is equivalent to, with probability `prob`, uniformly randomly
    /// applying either Pauli X, Y, or Z to `target_qubit`.
    ///
    /// This transforms `qureg = rho` into the mixed state:
    ///
    /// ```text
    /// (1 - prob) * rho  +  prob * 1/3 * (
    ///      X_q rho X_q +
    ///      Y_q rho Y_q +
    ///      Z_q rho Z_q
    /// )
    /// ```
    ///
    /// where `q = target_qubit`. The coefficient `prob` cannot exceed `3/4`, at
    /// which maximal mixing occurs.
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit upon which to induce depolarizing noise
    /// - `prob`: the probability of the depolarizing error occurring
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///   - if `prob` is not in `[0, 3/4]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// qureg.mix_depolarising(0, 0.75).unwrap();
    /// let amp = qureg.get_density_amp(0, 0).unwrap();
    ///
    /// assert!((amp.re - 0.5) < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_depolarising(
        &mut self,
        target_qubit: i32,
        prob: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixDepolarising(self.reg, target_qubit, prob);
        })
    }

    ///  Mixes a density matrix to induce single-qubit amplitude damping.
    ///
    /// With probability `prob`, applies damping (transition from `1` to `0`
    /// state). This transforms `qureg = rho` into the mixed state:
    ///
    /// ```text
    ///  K_0 rho K_0^\dagger + K_1 rho K_1^\dagger
    /// ```
    ///
    /// where `q = target_qubit` and `K_0` and `$K_1` are Kraus operators:
    ///
    /// ```text
    ///      K_0 =  [ 1       0       ]   K_1 = [ 0  sqrt(prob) ]
    ///             [ 0  sqrt(1-prob) ]         [ 0      0      ]
    /// ```
    ///
    /// The coefficient `prob` cannot exceed 1, at which total damping/decay
    /// occurs.
    ///
    /// Note that unlike [`mix_dephasing()`] and [`mix_depolarising()`], this
    /// function can increase the purity of a mixed state (by, as `prob` becomes
    /// `1`, gaining certainty that the qubit is in the 0 state).
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit upon which to induce amplitude damping
    /// - `prob`: the probability of the damping
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `target_qubit` is outside [0, [`num_qubits()`]).
    ///   - if `prob` is not in `[0, 1]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// qureg.mix_damping(0, 1.).unwrap();
    ///
    /// let amp = qureg.get_density_amp(0, 0).unwrap();
    /// assert!((amp.re - 1.) < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`mix_dephasing()`]: crate::Qureg::mix_dephasing()
    /// [`mix_depolarising()`]: crate::Qureg::mix_depolarising()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_damping(
        &mut self,
        target_qubit: i32,
        prob: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixDamping(self.reg, target_qubit, prob);
        })
    }

    /// Mixes a density matrix to induce two-qubit homogeneous depolarising
    /// noise.
    ///
    /// With probability `prob`, applies to `qubit1` and `qubit2` any operator
    /// of the set:
    ///
    /// ```text
    /// { IX, IY, IZ, XI, YI, ZI, XX, XY, XZ, YX, YY, YZ, ZX, ZY, ZZ }.
    /// ```
    ///
    /// Note this is the set of all two-qubit Pauli gates excluding `II`.
    ///
    /// # Parameters
    ///
    /// - `qubit1`: qubit upon which to induce depolarizing noise
    /// - `qubit2`: qubit upon which to induce depolarizing noise
    /// - `prob`: the probability of the phase error occurring
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `qubit1` or `qubit2` are outside [0, [`num_qubits()`]).
    ///   - if `qubit1 = qubit2`
    ///   - if `prob` is not in `[0, 15/16]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// qureg.mix_two_qubit_depolarising(0, 1, 15. / 16.).unwrap();
    ///
    /// let amp = qureg.get_density_amp(0, 0).unwrap();
    /// assert!((amp.re - 0.125).abs() < EPSILON);
    /// let amp = qureg.get_density_amp(0, 1).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_two_qubit_depolarising(
        &mut self,
        qubit1: i32,
        qubit2: i32,
        prob: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixTwoQubitDepolarising(self.reg, qubit1, qubit2, prob);
        })
    }

    /// Mixes a density matrix to induce general single-qubit Pauli noise.
    ///
    /// With probabilities `prob_x`, `prob_y` and `prob_z`, applies Pauli X, Y,
    /// and Z respectively to `target_qubit`.
    ///
    /// This function operates by first converting the given Pauli
    /// probabilities into a single-qubit Kraus map (four 2x2 operators).
    ///
    /// # Parameters
    ///
    /// - `target_qubit`: qubit to decohere
    /// - `prob_x`: the probability of inducing an X error
    /// - `prob_y`: the probability of inducing an Y error
    /// - `prob_z`: the probability of inducing an Z error
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` is not a density matrix
    ///   - if `target_qubit` is outside [0, [`num_qubits()`])
    ///   - if any of `prob_x`, `prob_y`, `prob_z` are not in `[0, 1]`
    ///   - if any of p in `{prob_x, prob_y or prob_z}` don't satisfy `p <= (1 -
    ///     prob_x - prob_y - prob_z)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let (prob_x, prob_y, prob_z) = (0.25, 0.25, 0.25);
    /// qureg.mix_pauli(0, prob_x, prob_y, prob_z).unwrap();
    ///
    /// let mut outcome_prob = -1.;
    /// let _ = qureg.measure_with_stats(0, &mut outcome_prob).unwrap();
    ///
    /// assert!((outcome_prob - 0.5).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_pauli(
        &mut self,
        target_qubit: i32,
        prob_x: Qreal,
        prob_y: Qreal,
        prob_z: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixPauli(self.reg, target_qubit, prob_x, prob_y, prob_z);
        })
    }

    /// Modifies register with `other_qureg`.
    ///
    /// The state becomes `(1-prob) * self +  prob * other_qureg`.  The
    /// probability `prob` must be in `[0,1]`.
    ///
    /// # Parameters
    ///
    /// - `prob`: the probability of `other_qureg` in the modified register
    /// - `other_qureg`: a density matrix to be mixed into
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `self` or `other_qureg` are not density matrices
    ///   - if the dimensions of `self` and `other_qureg` do not match
    ///   - if `prob` is not in `[0, 1]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut combine_qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// let other_qureg = {
    ///     let mut other_qureg = Qureg::try_new_density(2, &env)
    ///         .expect("cannot allocate memory for Qureg");
    ///     other_qureg.init_classical_state(3).unwrap();
    ///     other_qureg
    /// };
    ///
    /// combine_qureg.mix_density_matrix(0.5, &other_qureg).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_density_matrix(
        &mut self,
        prob: Qreal,
        other_qureg: &Qureg<'_>,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::mixDensityMatrix(self.reg, prob, other_qureg.reg);
        })
    }

    /// Calculate the purity of a density matrix.
    ///
    /// The purity of a density matrix is calculated by taking the trace of the
    /// density matrix squared. Returns `Tr (\rho^2)`.
    /// For a pure state, this =1.
    /// For a mixed state, the purity is less than 1 and is lower bounded by
    /// `1/2^n`, where n is the number of qubits. The minimum purity is achieved
    /// for the maximally mixed state `identity/2^n`.
    ///
    /// This function does not accept state-vectors, which clearly have purity
    /// 1.
    ///
    /// Note this function will give incorrect results for non-Hermitian Quregs
    /// (i.e. invalid density matrices), which will disagree with
    /// `Tr(\rho^2)`. Instead, this function returns `\sum_{ij}
    /// |\rho_{ij}|^2`.
    ///
    /// # Parameters
    ///
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuESTInputError`],
    ///
    /// - if the argument `qureg` is not a density matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let purity = qureg.calc_purity().unwrap();
    /// assert!((purity - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn calc_purity(&self) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe { ffi::calcPurity(self.reg) })
    }

    /// Calculates the fidelity of `qureg` (a state-vector or density matrix).
    ///
    /// Calculates the fidelity against a reference pure state (necessarily a
    /// state-vector).
    ///
    /// - If `qureg` is a state-vector, this function computes
    ///
    /// ```latex
    ///  |\langle \text{qureg} | \text{pure_state} \rangle|^2
    /// ```
    ///
    /// - If `qureg` is a density matrix, this function computes
    ///
    /// ```latex
    ///  \langle \text{pure_state} | \text{qureg} | \text{pure_state} \rangle
    /// ```
    ///
    /// In either case, the returned fidelity lies in `[0, 1]` (assuming both
    /// input states have valid normalisation). If any of the input `Qureg`s
    /// are not normalised, this function will return the real component of
    /// the correct linear algebra calculation.
    ///
    /// The number of qubits represented in `qureg` and `pure_state` must match.
    ///
    /// # Parameters
    ///
    /// - `pure_state`: a state vector
    ///
    /// Returns the fidelity between the input registers
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuESTInputError`],
    ///
    /// - if the second argument `pure_state` is not a state-vector
    /// - if the number of qubits `qureg` and `pure_state` do not match
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// let pure_state = {
    ///     let mut new_state =
    ///         Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///     new_state.init_plus_state();
    ///     new_state
    /// };
    ///
    /// let fidelity = qureg.calc_fidelity(&pure_state).unwrap();
    /// assert!((fidelity - 0.25).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn calc_fidelity(
        &self,
        pure_state: &Qureg<'_>,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::calcFidelity(self.reg, pure_state.reg)
        })
    }

    /// Performs a SWAP gate between `qubit1` and `qubit2`.
    ///
    /// This effects
    ///
    /// ```text
    /// [1 0 0 0]
    /// [0 0 1 0]
    /// [0 1 0 0]
    /// [0 0 0 1]
    /// ```
    ///
    /// on the designated qubits, though is performed internally by three CNOT
    /// gates.
    ///
    ///
    /// # Parameters
    ///
    /// - `qubit1`: qubit to swap
    /// - `qubit2`: other qubit to swap
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `qubit1` or `qubit2` is outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if `qubit1` and `qubit2` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// // init state |10>
    /// qureg.init_classical_state(1).unwrap();
    /// // swap to |01>
    /// qureg.swap_gate(0, 1).unwrap();
    ///
    /// let outcome = qureg.measure(0).unwrap();
    /// assert_eq!(outcome, 0);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn swap_gate(
        &mut self,
        qubit1: i32,
        qubit2: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::swapGate(self.reg, qubit1, qubit2);
        })
    }

    /// Performs a sqrt SWAP gate between `qubit1` and `qubit2`.
    ///
    /// This effects
    ///
    /// ```text
    /// [ 1     0        0     0 ]
    /// [ 0  (1+i)/2  (1-i)/2  0 ]
    /// [ 0  (1-i)/2  (1+i)/2  0 ]
    /// [ 0     0        0     1 ]
    /// ```
    ///
    /// on the designated qubits, though is performed internally by three CNOT
    /// gates.
    ///
    /// # Parameters
    ///
    /// - `qubit1`: qubit to sqrt swap
    /// - `qubit2`: other qubit to sqrt swap
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if either `qubit1` or `qubit2` is outside [0,
    ///     [`qureg.num_qubits()`]).
    ///   - if `qubit1` and `qubit2` are equal
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// // init state |10>
    /// qureg.init_classical_state(1).unwrap();
    /// qureg.sqrt_swap_gate(0, 1).unwrap();
    /// qureg.sqrt_swap_gate(0, 1).unwrap();
    /// let outcome = qureg.measure(0).unwrap();
    /// assert_eq!(outcome, 0);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn sqrt_swap_gate(
        &mut self,
        qb1: i32,
        qb2: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::sqrtSwapGate(self.reg, qb1, qb2);
        })
    }

    /// Apply a general single-qubit unitary with multiple control qubits.
    ///
    /// The operation is conditioned upon a specific bit sequence:
    /// `control_state`.
    ///
    /// Any number of control qubits can be specified, along with their
    /// classical state (`0` or `1`) to condition upon. Only amplitudes of
    /// computational basis states  for which `control_qubits` have
    /// corresponding bit values `control_state` are modified  by `u`.
    ///  
    /// This function is equivalent (albeit faster) to applying [`pauli_x()`] on
    /// each of the control qubits which are conditioned on outcome `0`,
    /// calling [`multi_controlled_unitary()`], then re-appplying
    /// `pauli_x()` on the same qubits.
    ///
    ///  # Parameters
    ///
    /// - `control_qubits`: the indices of the control qubits
    /// - `control_state`: the bit values (`0` or `1`) of each control qubit,
    ///   upon which to condition
    /// - `target_qubit`: qubit to operate the unitary upon
    /// - `u`: single-qubit unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit index (`target_qubit` or one in `control_qubits`) is
    ///     outside [0, [`num_qubits()`]),
    ///   - if any qubit in `control_qubits` is repeated
    ///   - if `control_qubits` contains `target_qubit`
    ///   - if any element of `control_state` is not a bit (`0` or `1`)
    ///   - if `u` is not unitary
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let control_qubits = &[1, 2];
    /// let control_state = &[0, 0];
    /// let target_qubit = 0;
    /// let u = &ComplexMatrix2::new([[0., 1.], [1., 0.]], [[0., 0.], [0., 0.]]);
    /// qureg
    ///     .multi_state_controlled_unitary(
    ///         control_qubits,
    ///         control_state,
    ///         target_qubit,
    ///         u,
    ///     )
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`pauli_x()`]: Qureg::pauli_x()
    /// [`multi_controlled_unitary()`]: Qureg::multi_controlled_unitary()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_state_controlled_unitary(
        &mut self,
        control_qubits: &[i32],
        control_state: &[i32],
        target_qubit: i32,
        u: &ComplexMatrix2,
    ) -> Result<(), QuestError> {
        let num_control_qubits = control_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiStateControlledUnitary(
                self.reg,
                control_qubits.as_ptr(),
                control_state.as_ptr(),
                num_control_qubits,
                target_qubit,
                u.0,
            );
        })
    }

    /// Apply a multi-qubit Z rotation on selected qubits.
    ///
    /// This is the unitary
    ///
    /// ```latex
    ///    \exp \left( - i \, \frac{\theta}{2} \; \bigotimes_{j}^{\text{numQubits}} Z_j\right)
    /// ```
    ///
    /// where the Pauli Z gates operate the qubits listed in `qubits`, and cause
    /// rotations of `theta = angle`.
    ///
    /// All qubits not appearing in `qubits` are assumed to receive the
    /// identity operator.
    ///
    /// This has the effect of premultiplying every amplitude with `exp(+/- i
    /// \theta/2)` where the sign is determined by the parity of the target
    /// qubits for that amplitude.
    ///
    ///  # Parameters
    ///
    /// - `qubits`: a list of the indices of the target qubits
    /// - `angle`: the angle by which the multi-qubit state is rotated around
    ///   the Z axis
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit index in `qubits` is outside [0, [`num_qubits()`]),
    ///   - if any qubit in `qubits` is repeated
    ///   - if `control_qubits` contains `target_qubit`
    ///   - if any element of `control_state` is not a bit (`0` or `1`)
    ///   - if `u` is not unitary
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.init_plus_state();
    ///
    /// let qubits = &[0, 1];
    /// let angle = PI;
    /// qureg.multi_rotate_z(qubits, angle).unwrap();
    ///
    /// let amp = qureg.get_imag_amp(0).unwrap();
    /// assert!((amp + 0.5).abs() < EPSILON);
    /// let amp = qureg.get_imag_amp(1).unwrap();
    /// assert!((amp - 0.5).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_rotate_z(
        &mut self,
        qubits: &[i32],
        angle: Qreal,
    ) -> Result<(), QuestError> {
        let num_qubits = qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiRotateZ(self.reg, qubits.as_ptr(), num_qubits, angle);
        })
    }

    /// Apply a multi-qubit multi-Pauli rotation.
    ///
    /// This is the unitary
    ///
    /// ```latex
    ///    \exp \left( - i \, \frac{\theta}{2} \; \bigotimes_{j}^{\text{numTargets}} \hat{\sigma}_j\right)
    /// ```
    ///
    ///  where  `theta = angle` and `$\hat{\sigma}_j \in \{X, Y, Z\}$` is a
    /// Pauli operator [`PauliOpType`] operating upon the corresponding qubit
    /// `target_qubits`.
    ///
    /// This function effects the Pauli gadget by first rotating the qubits
    /// which are nominated to receive `X` or `Y` Paulis into alternate
    /// basis, performing [`multi_rotate_z()`] on all target qubits, then
    /// restoring the original basis.
    ///
    ///  # Parameters
    ///
    /// - `target_qubits`: a list of the indices of the target qubits
    /// - `target_paulis`: a list of the Pauli operators `PauliOpType`
    /// - `angle`: the angle by which the multi-qubit state is rotated
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit index in `target_qubits` is outside [0,
    ///     [`num_qubits()`]),
    ///   - if any qubit in `target_qubits` is repeated
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::PAULI_X;
    ///
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let target_qubits = &[1, 2];
    /// let target_paulis = &[PAULI_X, PAULI_X];
    /// let angle = PI;
    ///
    /// qureg
    ///     .multi_rotate_pauli(target_qubits, target_paulis, angle)
    ///     .unwrap();
    ///
    /// let amp = qureg.get_imag_amp(6).unwrap();
    /// assert!((amp + 1.).abs() < 2. * EPSILON);
    /// ```
    ///
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`PauliOpType`]: crate::PauliOpType
    /// [`multi_rotate_z()`]: Qureg::multi_rotate_z()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_rotate_pauli(
        &mut self,
        target_qubits: &[i32],
        target_paulis: &[PauliOpType],
        angle: Qreal,
    ) -> Result<(), QuestError> {
        let num_targets = target_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiRotatePauli(
                self.reg,
                target_qubits.as_ptr(),
                target_paulis.as_ptr(),
                num_targets,
                angle,
            );
        })
    }

    /// Apply a multi-controlled multi-target Z rotation.
    ///
    /// All qubits not appearing in `target_qubits` and `control_qubits` are
    /// assumed to receive the identity operator.
    ///
    /// This has the effect of premultiplying all amplitudes (for which the
    /// control qubits are `1`)  with `$\exp(\pm i \theta/2)$`, where the
    /// sign is determined by the parity of  the target qubits for that
    /// amplitude.
    ///
    ///  # Parameters
    ///
    /// - `control_qubits`: list of the indices of qubits to control upon
    /// - `target_qubits`: a list of the indices of the target qubits
    /// - `angle`: the angle by which the multi-qubit state is rotated
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit index in `target_qubits` or `control_qubits` is outside
    ///     [0, [`num_qubits()`]),
    ///   - if `control_qubits` or `target_qubits` contain any repetitions
    ///   - if any qubit in `control_qubits` is also in `target_qubits` (and
    ///     vice versa)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    ///
    /// // Initialize `|1111>`
    /// (0..4).try_for_each(|i| qureg.pauli_x(i)).unwrap();
    ///
    /// let control_qubits = &[0, 1];
    /// let target_qubits = &[2, 3];
    /// let angle = 2. * PI;
    /// qureg
    ///     .multi_controlled_multi_rotate_z(control_qubits, target_qubits, angle)
    ///     .unwrap();
    ///
    /// // the state is now `-1. * |1111>`
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp + 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_multi_rotate_z(
        &mut self,
        control_qubits: &[i32],
        target_qubits: &[i32],
        angle: Qreal,
    ) -> Result<(), QuestError> {
        let num_controls = control_qubits.len() as i32;
        let num_targets = target_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledMultiRotateZ(
                self.reg,
                control_qubits.as_ptr(),
                num_controls,
                target_qubits.as_ptr(),
                num_targets,
                angle,
            );
        })
    }

    /// Apply a multi-controlled multi-target multi-Pauli rotation.
    ///
    /// All qubits not appearing in `target_qubits` and `control_qubits` are
    /// assumed to receive the identity operator.
    ///
    /// This function effects the controlled Pauli gadget by first (controlled)
    /// rotating the qubits which are targeted with either `X` or `Y` into
    /// alternate basis, performing [`multi_controlled_multi_rotate_z()`] on all
    /// target qubits, then restoring the original basis.
    ///
    /// # Parameters
    ///
    /// - `control_qubits`: list of the indices of qubits to control upon
    /// - `target_qubits`: a list of the indices of the target qubits
    /// - `target_paulis`: a list of the Pauli operators [`PauliOpType`]
    /// - `angle`: the angle by which the multi-qubit state is rotated
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit index in `target_qubits` or `control_qubits` is outside
    ///     [0, [`num_qubits()`]),
    ///   - if `control_qubits` or `target_qubits` contain any repetitions
    ///   - if any qubit in `control_qubits` is also in `target_qubits` (and
    ///     vice versa)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::PAULI_Z;
    ///
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    ///
    /// // Initialize `|1111>`
    /// (0..4).try_for_each(|i| qureg.pauli_x(i)).unwrap();
    ///
    /// let control_qubits = &[0, 1];
    /// let target_qubits = &[2, 3];
    /// let target_paulis = &[PAULI_Z, PAULI_Z];
    /// let angle = 2. * PI;
    /// qureg
    ///     .multi_controlled_multi_rotate_pauli(
    ///         control_qubits,
    ///         target_qubits,
    ///         target_paulis,
    ///         angle,
    ///     )
    ///     .unwrap();
    ///
    /// // the state is now `-1. * |1111>`
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp + 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`PauliOpType`]: crate::PauliOpType
    /// [`multi_controlled_multi_rotate_z()`]: Qureg::multi_controlled_multi_rotate_z()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_multi_rotate_pauli(
        &mut self,
        control_qubits: &[i32],
        target_qubits: &[i32],
        target_paulis: &[PauliOpType],
        angle: Qreal,
    ) -> Result<(), QuestError> {
        let num_controls = control_qubits.len() as i32;
        let num_targets = target_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledMultiRotatePauli(
                self.reg,
                control_qubits.as_ptr(),
                num_controls,
                target_qubits.as_ptr(),
                target_paulis.as_ptr(),
                num_targets,
                angle,
            );
        })
    }

    /// Computes the expected value of a product of Pauli operators.
    ///
    /// Letting `$\sigma = \otimes_j \hat{\sigma}_j$` be the operators
    /// indicated by `pauli_codes` and acting on qubits `target_qubits`,
    /// this function computes `$ \langle \psi | \sigma | \psi \rangle $`
    /// if `$qureg = \psi$` is a state-vector, and computes `$ \text{Tr}(\sigma
    /// \rho) $`  if `$qureg = \rho $` is a density matrix.
    ///  
    /// The slice `pauli_codes` specifies which Pauli operators to enact on the
    /// corresponding qubits in `target_qubits`. The target qubits must be
    /// unique, and at most `self.num_qubits()` may be specified.
    ///
    /// For example, on a 7-qubit state-vector,  
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// # use PauliOpType::{
    /// #     PAULI_X,
    /// #     PAULI_Z,
    /// #     PAULI_Y,
    /// #     PAULI_I,
    /// # };
    /// # let env = QuestEnv::new();
    /// # let qureg = {
    /// #     let mut qureg = Qureg::try_new(2, &env).expect(
    /// #         "cannot allocate memory for Qureg",
    /// #     );
    /// #     qureg.init_plus_state();
    /// #     qureg
    /// # };
    /// # let mut workspace = Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// qureg.calc_expec_pauli_prod(&[4,5,6], &[PAULI_X, PAULI_I, PAULI_Z], &mut workspace);
    /// ```
    ///
    /// will compute `$ \langle \psi | I I I I X I Z | \psi \rangle $` (where in
    /// this notation, the left-most operator applies to the
    /// least-significant qubit, i.e. that with index `0`).
    ///
    /// `workspace` must be a register with the same type (state-vector vs
    /// density matrix) and dimensions (number of represented qubits) as `self`,
    /// and is used as working space. When this function returns, `self` will
    /// be unchanged and `workspace` will be set to `$ \sigma
    /// | \psi \rangle $` (if `self` is a state-vector)  or $ \sigma
    /// \rho $` (if `self` is a density matrix).
    ///
    /// NOTE that this last quantity is NOT the result of applying  the paulis
    /// as unitaries, `$ \sigma^\dagger \rho \sigma $`, but is instead the
    /// result of their direct multiplication with the density matrix. It
    /// is therefore itself not a valid density matrix.
    ///
    /// This function works by cloning the `self` state into `workspace`,
    /// applying the specified  Pauli operators to `workspace` then
    /// computing its inner product with `self` (for state-vectors)
    /// or its trace (for density matrices). It therefore should scale linearly
    /// in time with the number of specified non-identity Pauli operators,
    /// which is bounded by the number of represented qubits.
    ///
    /// # Parameters
    ///
    /// - `target_qubits`: a list of the indices of the target qubits
    /// - `pauli_codes`: a list of the Pauli codes to apply to the corresponding
    ///   qubits in `target_qubits`
    /// - `workspace`: a working-space qureg with the same dimensions as `self`,
    ///   which is modified to be the result of multiplying the state with the
    ///   pauli operators
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubits.len() > self.num_qubits()`
    ///   - if any qubit index in `target_qubits` is outside [0,
    ///     [`num_qubits()`]),
    ///   - if `target_qubits` contain any repetitions
    ///   - if `workspace` is not of the same dimension as `self`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::PAULI_X;
    ///
    /// let env = QuestEnv::new();
    /// let qureg = {
    ///     let mut qureg = Qureg::try_new(2, &env).expect(
    ///         "cannot allocate memory for
    /// Qureg",
    ///     );
    ///     qureg.init_plus_state();
    ///     qureg
    /// };
    /// let mut workspace =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let target_qubits = &[0, 1];
    /// let pauli_codes = &[PAULI_X, PAULI_X];
    ///
    /// let product = qureg
    ///     .calc_expec_pauli_prod(target_qubits, pauli_codes, &mut workspace)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn calc_expec_pauli_prod(
        &self,
        target_qubits: &[i32],
        pauli_codes: &[PauliOpType],
        workspace: &mut Qureg<'_>,
    ) -> Result<Qreal, QuestError> {
        let num_targets = target_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::calcExpecPauliProd(
                self.reg,
                target_qubits.as_ptr(),
                pauli_codes.as_ptr(),
                num_targets,
                workspace.reg,
            )
        })
    }

    /// Computes the expected value of a sum of products of Pauli operators.
    ///
    /// Let
    ///
    /// ```latex
    /// H = \sum_i c_i \otimes_j^{N} \hat{\sigma}_{i,j}
    /// ```
    ///
    /// be the operators indicated by `all_pauli_codes` (where `$ c_i \in $`
    /// `term_coeffs`  and `N = self.num_qubits()`).
    ///
    /// This function computes `$ \langle \psi | H | \psi \rangle $` if `self =
    /// \psi $` is a state-vector, and computes `$ \text{Tr}(H \rho)
    /// =\text{Trace}(\rho H) $`  if `self = \rho $` is a density matrix.
    ///
    /// `all_pauli_codes` is an array which specifies which Pauli operators to
    /// apply. For each sum term, a Pauli operator must be specified for
    /// EVERY qubit in `self`; each set of `term_coeffs.len()` operators will be
    /// grouped into a product.  `term_coeffs` is an arrray containing the term
    /// coefficients.
    ///
    /// For example, on a 3-qubit state-vector,  
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// # use PauliOpType::{
    /// #     PAULI_X,
    /// #     PAULI_Z,
    /// #     PAULI_Y,
    /// #     PAULI_I,
    /// # };
    /// # let env = QuestEnv::new();
    /// # let qureg = {
    /// #     let mut qureg = Qureg::try_new(2, &env).expect(
    /// #         "cannot allocate memory for Qureg",
    /// #     );
    /// #     qureg.init_plus_state();
    /// #     qureg
    /// # };
    /// # let mut workspace =
    /// #     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    /// let paulis = [PAULI_X, PAULI_I, PAULI_I,  PAULI_X, PAULI_Y, PAULI_Z];
    /// let coeffs = [1.5, -3.6];
    /// qureg.calc_expec_pauli_sum(&paulis, &coeffs, &mut workspace);
    /// ```
    ///
    /// will compute `$ \langle \psi | (1.5 X I I - 3.6 X Y Z) | \psi \rangle$`
    /// (where in this notation, the left-most operator  applies to the
    /// least-significant qubit, i.e. that with index `0`).  
    /// `workspace` must be a register with the same type (state-vector vs
    /// density matrix) and dimensions (number of represented qubits) as
    /// `self`, and is used as working space. When this function returns,
    /// `self`  will be unchanged and `workspace` will be set to `self`
    /// pre-multiplied with the final Pauli product.
    ///
    /// NOTE that if `self` is a density matrix, `workspace` will become `$
    /// \hat{\sigma} \rho $` which is itself not a density matrix (it is
    /// distinct from `$ \hat{\sigma} \rho \hat{\sigma}^\dagger $`).
    ///
    /// This function works by cloning the `self` state into `workspace`,
    /// applying each of the specified  Pauli products to `workspace` (one
    /// Pauli operation at a time), then computing its inner product with `self`
    /// (for state-vectors) or its trace (for density matrices) multiplied
    /// with the corresponding coefficient, and summing these contributions.
    /// It therefore should scale linearly in time with the total number of
    /// non-identity specified Pauli operators.
    ///
    /// # Parameters
    ///
    /// - `all_pauli_codes`: a list of the Pauli codes of all Paulis involved in
    ///   the products of terms. A Pauli must be specified for each qubit  in
    ///   the register, in every term of the sum.
    /// - `term_coeffs`: The coefficients of each term in the sum of Pauli
    ///   products
    /// - `workspace`: a working-space qureg with the same dimensions as `self`,
    ///   which is modified to be the result of multiplying the state with the
    ///   pauli operators
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `workspace` is not of the same dimension as `self`
    ///
    /// # Examples
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::{
    ///     PAULI_X,
    ///     PAULI_Z,
    /// };
    ///
    /// let env = QuestEnv::new();
    /// let qureg = {
    ///     let mut qureg = Qureg::try_new(2, &env).expect(
    ///         "cannot allocate memory for
    /// Qureg",
    ///     );
    ///     qureg.init_plus_state();
    ///     qureg
    /// };
    /// let mut workspace =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let all_pauli_codes = &[PAULI_X, PAULI_Z, PAULI_Z, PAULI_X];
    /// let term_coeffs = &[0.5, 0.5];
    ///
    /// let sum = qureg
    ///     .calc_expec_pauli_sum(all_pauli_codes, term_coeffs, &mut workspace)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn calc_expec_pauli_sum(
        &self,
        all_pauli_codes: &[PauliOpType],
        term_coeffs: &[Qreal],
        workspace: &mut Qureg<'_>,
    ) -> Result<Qreal, QuestError> {
        let num_sum_terms = term_coeffs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::calcExpecPauliSum(
                self.reg,
                all_pauli_codes.as_ptr(),
                term_coeffs.as_ptr(),
                num_sum_terms,
                workspace.reg,
            )
        })
    }

    /// Computes the expected value under Hermitian operator.
    ///
    /// Represent `hamil` as `$ H = \sum_i c_i \otimes_j^{N}
    /// \hat{\sigma}_{i,j} $`  and `N = hamil.num_qubits()`.  This function
    /// computes:
    ///
    /// ```latex
    ///  \langle \psi | H | \psi \rangle
    /// ```
    ///
    ///  if `self =  \psi` is a state-vector, and computes
    ///
    /// ```latex
    /// \text{Trace}(H \rho) =\text{Trace}(\rho H)
    /// ```
    ///
    /// if `self = \rho`  is a density matrix.
    ///
    /// This function is merely an encapsulation of `calc_expec_pauli_sum()` -
    /// refer to the doc there for an elaboration.
    ///
    /// `workspace` must be a register with the same type (state-vector vs
    /// density matrix) and dimensions (number of represented qubits) as
    /// `self` and `hamil`, and is used as working space.  When this
    /// function returns, `self`  will be unchanged and `workspace` will be
    /// set to `self` pre-multiplied with the final Pauli product in
    /// `hamil`.
    ///
    /// Note that if `self` is a density matrix, `workspace` will
    /// become `$ \hat{\sigma} \rho $`  which is itself not a density
    /// matrix (it is distinct from `$\hat{\sigma} \rho \hat{\sigma}^\dagger$`).
    ///
    /// This function works by cloning the `self` state into `workspace`,
    /// applying each of the specified  Pauli products in `hamil` to
    /// `workspace` (one Pauli operation at a time), then computing its inner
    /// product with `self` (for state-vectors) or its trace (for density
    /// matrices) multiplied with the corresponding coefficient, and summing
    /// these contributions.  It therefore should scale linearly in time
    /// with the total number of non-identity specified Pauli operators.
    ///
    /// # Parameters
    ///
    /// - `hamil`: a [`PauliHamil`]
    /// - `workspace`: a working-space `Qureg` with the same dimensions as
    ///   `self`, which is modified to be the result of multiplying the state
    ///   with the final specified Pauli product
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `workspace` is not of the same dimension as `self` and `hamil`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::{
    ///     PAULI_X,
    ///     PAULI_Z,
    /// };
    ///
    /// let env = QuestEnv::new();
    /// let qureg = {
    ///     let mut qureg =
    ///         Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///     qureg.init_plus_state();
    ///     qureg
    /// };
    /// let mut workspace =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let hamil = &mut PauliHamil::try_new(2, 2).unwrap();
    /// init_pauli_hamil(hamil, &[0.5, 0.5], &[PAULI_X, PAULI_X, PAULI_X, PAULI_Z])
    ///     .unwrap();
    ///
    /// let expec_val =
    ///     qureg.calc_expec_pauli_hamil(hamil, &mut workspace).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`PauliHamil`]: crate::PauliHamil
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn calc_expec_pauli_hamil(
        &self,
        hamil: &PauliHamil,
        workspace: &mut Qureg<'_>,
    ) -> Result<Qreal, QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::calcExpecPauliHamil(self.reg, hamil.0, workspace.reg)
        })
    }

    ///  Apply a general two-qubit unitary (including a global phase factor).
    ///
    /// `target_qubit1` is treated as the least significant qubit in `u`,
    /// such that a row in `u` is dotted with the vector
    ///
    /// ```latex
    ///  |\text{targetQubit2} \;\; \text{targetQubit1}\rangle : \{|00\rangle, |01\rangle, |10\rangle, |11\rangle \}
    /// ```
    ///
    /// The passed `ComplexMatrix4` must be unitary, otherwise an error is
    /// thrown.  Use [`Qureg::apply_matrix4()`] to left-multiply a non-unitary
    /// `ComplexMatrix4`.
    ///
    /// Note that in distributed mode, this routine requires that each node
    /// contains at least 4 amplitudes.  This means an q-qubit register
    /// (state vector or density matrix) can be distributed by at most
    /// `2^(q/4)` nodes.
    ///
    /// # Parameters
    ///
    /// - `target_qubit1`: first qubit to operate on, treated as least
    ///   significant in `u`
    /// - `target_qubit2`: first qubit to operate on, treated as most
    ///   significant in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit1` or `target_qubit2` are outside `[0,
    ///     self.num_qubits())`
    ///   - if `target_qubit1` equals `target_qubit2`
    ///   - if matrix `u` is not unitary
    ///   - if each node cannot fit 4 amplitudes in distributed mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    ///
    /// let target_qubit1 = 1;
    /// let target_qubit2 = 2;
    /// let u = &ComplexMatrix4::new(
    ///     [
    ///         [0., 0., 0., 1.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [1., 0., 0., 0.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    ///
    /// qureg
    ///     .two_qubit_unitary(target_qubit1, target_qubit2, u)
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(7).unwrap();
    /// assert!((amp - 1.) < EPSILON);
    /// ```
    ///
    /// See [`QuEST` Aqureg.PI] for more information.
    ///
    /// [`Qureg::apply_matrix4()`]: crate::Qureg::apply_matrix4()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn two_qubit_unitary(
        &mut self,
        target_qubit1: i32,
        target_qubit2: i32,
        u: &ComplexMatrix4,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::twoQubitUnitary(self.reg, target_qubit1, target_qubit2, u.0);
        })
    }

    /// Apply a general controlled two-qubit unitary.
    ///
    /// The given unitary is applied to the target amplitudes where the control
    /// qubit has value 1.
    ///
    /// `target_qubit1` is treated as the least significant qubit in `u`,
    /// such that a row in `u` is dotted with the vector
    ///
    /// The passed `ComplexMatrix4` must be unitary, otherwise an error is
    /// thrown.
    ///
    /// Note that in distributed mode, this routine requires that each node
    /// contains at least 4 amplitudes.  This means an q-qubit register
    /// (state vector or density matrix) can be distributed by at most
    /// `2^(q/4)` nodes.
    ///
    /// # Parameters
    ///
    /// - `contol_qubit`:  the control qubit which must be in state 1 to effect
    ///   the given unitary
    /// - `target_qubit1`: first qubit to operate on, treated as least
    ///   significant in `u`
    /// - `target_qubit2`: first qubit to operate on, treated as most
    ///   significant in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `control_qubit`,  `target_qubit1` or `target_qubit2` are outside
    ///     `[0, self.num_qubits())`
    ///   - if any of `control_qubit`, `target_qubit1` and `target_qubit2` are
    ///     equal
    ///   - if matrix `u` is not unitary
    ///   - if each node cannot fit 4 amplitudes in distributed mode
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    ///
    /// let control_qubit = 0;
    /// let target_qubit1 = 1;
    /// let target_qubit2 = 2;
    /// let u = &ComplexMatrix4::new(
    ///     [
    ///         [0., 0., 0., 1.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [1., 0., 0., 0.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    ///
    /// qureg
    ///     .controlled_two_qubit_unitary(
    ///         control_qubit,
    ///         target_qubit1,
    ///         target_qubit2,
    ///         u,
    ///     )
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(7).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`Qureg::apply_matrix4()`]: crate::Qureg::apply_matrix4()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_two_qubit_unitary(
        &mut self,
        control_qubit: i32,
        target_qubit1: i32,
        target_qubit2: i32,
        u: &ComplexMatrix4,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::controlledTwoQubitUnitary(
                self.reg,
                control_qubit,
                target_qubit1,
                target_qubit2,
                u.0,
            );
        })
    }

    /// Apply a general multi-qubit unitary with any number of target qubits.
    ///
    /// Any number of control qubits can be specified, and if all have value 1,
    /// the given unitary is applied to the target qubit.
    ///
    /// `target_qubit1` is treated as the least significant qubit in `u`,
    /// such that a row in `u` is dotted with the vector
    ///
    /// ```latex
    ///  |\text{targetQubit2} \;\; \text{targetQubit1}\rangle : \{|00\rangle, |01\rangle, |10\rangle, |11\rangle \}
    /// ```
    ///
    /// The passed `ComplexMatrix4` must be unitary, otherwise an error is
    /// thrown.  Use [`Qureg::apply_matrix4()`] to left-multiply a non-unitary
    /// `ComplexMatrix4`.
    ///
    /// Note that in distributed mode, this routine requires that each node
    /// contains at least 4 amplitudes.  This means an q-qubit register
    /// (state vector or density matrix) can be distributed by at most
    ///
    /// # Parameters
    ///
    /// - `contol_qubits`:  the control qubits which all must be in state 1 to
    ///   effect the given unitary
    /// - `target_qubit1`: first qubit to operate on, treated as least
    ///   significant in `u`
    /// - `target_qubit2`: first qubit to operate on, treated as most
    ///   significant in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `target_qubit1` or `target_qubit2` are outside `[0,
    ///     self.num_qubits())`
    ///   - if `target_qubit1` equals `target_qubit2`
    ///   - if any qubit in `control_qubits` is outside `[0, self.num_qubits())`
    ///   - if `control_qubits` are not unique
    ///   - if either `target_qubit1` or `target_qubit2` are in `control_qubit``
    ///   - if matrix `u` is not unitary
    ///   - if each node cannot fit 4 amplitudes in distributed mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let control_qubits = &[0, 1];
    /// let target_qubit1 = 2;
    /// let target_qubit2 = 3;
    /// let u = &ComplexMatrix4::new(
    ///     [
    ///         [0., 0., 0., 1.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [1., 0., 0., 0.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    ///
    /// qureg
    ///     .multi_controlled_two_qubit_unitary(
    ///         control_qubits,
    ///         target_qubit1,
    ///         target_qubit2,
    ///         u,
    ///     )
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_two_qubit_unitary(
        &mut self,
        control_qubits: &[i32],
        target_qubit1: i32,
        target_qubit2: i32,
        u: &ComplexMatrix4,
    ) -> Result<(), QuestError> {
        let num_control_qubits = control_qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledTwoQubitUnitary(
                self.reg,
                control_qubits.as_ptr(),
                num_control_qubits,
                target_qubit1,
                target_qubit2,
                u.0,
            );
        })
    }

    /// Apply a general multi-qubit unitary with any number of target qubits.
    ///
    /// The first target qubit in `targs` is treated as least significant in
    /// `u`.
    ///
    /// The passed `ComplexMatrixN` must be unitary and be a compatible size
    /// with the specified number of target qubits, otherwise an error is
    /// thrown.
    ///
    /// To left-multiply a non-unitary `ComplexMatrixN`, use
    /// [`apply_matrix_n()`].
    ///
    /// Note that in multithreaded mode, each thread will clone
    /// `2^(targs.len())` amplitudes, and store these in the runtime stack.
    /// Using `t` threads, the total memory overhead of this function is
    /// `t*2^(targs.len())`. For many targets (e.g. 16 qubits), this may
    /// cause a stack-overflow / seg-fault (e.g. on a 1 MiB stack).
    ///  
    /// Note too that in distributed mode, this routine requires that each node
    /// contains at least `2^(targs.len())` amplitudes in the register. This
    /// means an q-qubit register (state vector or density matrix)
    /// can be distributed by at most `2^q / 2^(targs.len())` nodes.
    ///
    /// # Parameters
    ///
    /// - `targs`: a list of the target qubits, ordered least significant to
    ///   most in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any index in `targs` is outside of `[0, self.num_qubits())`
    ///   - if `targs` are not unique
    ///   - if matrix `u` is not unitary
    ///   - if `u` is not of a compatible size with `targs.len()`
    ///   - if a node cannot fit the required number of target amplitudes in
    ///     distributed mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let u = &mut ComplexMatrixN::try_new(2).unwrap();
    /// let zero_row = &[0., 0., 0., 0.];
    /// init_complex_matrix_n(
    ///     u,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[1., 0., 0., 0.],
    ///     ],
    ///     &[zero_row, zero_row, zero_row, zero_row],
    /// )
    /// .unwrap();
    ///
    /// qureg.multi_qubit_unitary(&[0, 1], u).unwrap();
    ///
    /// // Check if the register is now in the state `|11>`
    /// let amp = qureg.get_real_amp(3).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`apply_matrix_n()`]: crate::Qureg::apply_matrix_n()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_qubit_unitary(
        &mut self,
        targs: &[i32],
        u: &ComplexMatrixN,
    ) -> Result<(), QuestError> {
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiQubitUnitary(self.reg, targs.as_ptr(), num_targs, u.0);
        })
    }

    /// Apply a general controlled multi-qubit unitary (including a global phase
    /// factor).
    ///
    /// One control and any number of target qubits can be specified.
    ///
    /// The target qubits in `targs` are treated as ordered least significant
    /// to most significant in `u`.
    ///
    /// The passed `ComplexMatrixN` must be unitary and be a compatible size
    /// with the specified number of target qubits, otherwise an error is
    /// thrown.
    ///
    /// Note that in multithreaded mode, each thread will clone
    /// `2^(targs.len())` amplitudes, and store these in the runtime stack.
    /// Using `t` threads, the total memory overhead of this function is
    /// `t*2^(targs.len())`. For many targets (e.g. 16 qubits), this may
    /// cause a stack-overflow / seg-fault (e.g. on a 1 MiB stack).
    ///  
    /// Note too that in distributed mode, this routine requires that each node
    /// contains at least `2^(targs.len())` amplitudes in the register. This
    /// means an q-qubit register (state vector or density matrix)
    /// can be distributed by at most `2^q / 2^(targs.len())` nodes.
    ///
    /// # Parameters
    ///
    /// - `ctrl`: the control qubit
    /// - `targs`: a list of the target qubits, ordered least significant to
    ///   most in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `ctrl` or any index in `targs` is outside of `[0,
    ///     self.num_qubits())`
    ///   - if `targs` are not unique
    ///   - if matrix `u` is not unitary
    ///   - if `u` is not of a compatible size with `targs.len()`
    ///   - if a node cannot fit the required number of target amplitudes in
    ///     distributed mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    ///
    /// let u = &mut ComplexMatrixN::try_new(2).unwrap();
    /// let zero_row = &[0., 0., 0., 0.];
    /// init_complex_matrix_n(
    ///     u,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[1., 0., 0., 0.],
    ///     ],
    ///     &[zero_row, zero_row, zero_row, zero_row],
    /// )
    /// .unwrap();
    ///
    /// let ctrl = 0;
    /// let targs = &[1, 2];
    /// qureg
    ///     .controlled_multi_qubit_unitary(ctrl, targs, u)
    ///     .unwrap();
    ///
    /// // Check if the register is now in the state `|111>`
    /// let amp = qureg.get_real_amp(7).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`Qureg::apply_matrix_n()`]: crate::Qureg::apply_matrix_n()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn controlled_multi_qubit_unitary(
        &mut self,
        ctrl: i32,
        targs: &[i32],
        u: &ComplexMatrixN,
    ) -> Result<(), QuestError> {
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::controlledMultiQubitUnitary(
                self.reg,
                ctrl,
                targs.as_ptr(),
                num_targs,
                u.0,
            );
        })
    }

    /// Apply a general multi-controlled multi-qubit unitary (including a global
    /// phase factor).
    ///
    /// Any number of control and target qubits can be specified.
    ///
    /// The target qubits in `targs` are treated as ordered least significant
    /// to most significant in `u`.
    ///
    /// The passed `ComplexMatrixN` must be unitary and be a compatible size
    /// with the specified number of target qubits, otherwise an error is
    /// thrown.
    ///
    /// Note that in multithreaded mode, each thread will clone
    /// `2^(targs.len())` amplitudes, and store these in the runtime stack.
    /// Using `t` threads, the total memory overhead of this function is
    /// `t*2^(targs.len())`. For many targets (e.g. 16 qubits), this may
    /// cause a stack-overflow / seg-fault (e.g. on a 1 MiB stack).
    ///  
    /// Note too that in distributed mode, this routine requires that each node
    /// contains at least `2^(targs.len())` amplitudes in the register. This
    /// means an q-qubit register (state vector or density matrix)
    /// can be distributed by at most `2^q / 2^(targs.len())` nodes.
    ///
    /// # Parameters
    ///
    /// - `ctrls`: a list  of control qubits
    /// - `targs`: a list of the target qubits, ordered least significant to
    ///   most in `u`
    /// - `u`: unitary matrix to apply
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit in `ctrls` or `targs` is outside of `[0,
    ///     self.num_qubits())`
    ///   - if `ctrls` or `targs` contain any repetitions
    ///   - if any qubit in `ctrls` is also in `targs` (and vice versa)
    ///   - if `targs.len() < 1`
    ///   - if `ctrls.len() < 1` (use [`multi_qubit_unitary()`] for no controls)
    ///   - if matrix `u` is not unitary
    ///   - if `u` is not of a compatible size with `targs.len()`
    ///   - if a node cannot fit the required number of target amplitudes in
    ///     distributed mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let u = &mut ComplexMatrixN::try_new(2).unwrap();
    /// let zero_row = &[0., 0., 0., 0.];
    /// init_complex_matrix_n(
    ///     u,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[1., 0., 0., 0.],
    ///     ],
    ///     &[zero_row, zero_row, zero_row, zero_row],
    /// )
    /// .unwrap();
    ///
    /// let ctrls = &[0, 1];
    /// let targs = &[2, 3];
    /// qureg
    ///     .multi_controlled_multi_qubit_unitary(ctrls, targs, u)
    ///     .unwrap();
    ///
    /// // Check if the register is now in the state `|1111>`
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`Qureg::apply_matrix_n()`]: crate::Qureg::apply_matrix_n()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`multi_qubit_unitary()`]: crate::Qureg::multi_qubit_unitary()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn multi_controlled_multi_qubit_unitary(
        &mut self,
        ctrls: &[i32],
        targs: &[i32],
        u: &ComplexMatrixN,
    ) -> Result<(), QuestError> {
        let num_ctrls = ctrls.len() as i32;
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::multiControlledMultiQubitUnitary(
                self.reg,
                ctrls.as_ptr(),
                num_ctrls,
                targs.as_ptr(),
                num_targs,
                u.0,
            );
        })
    }

    /// Apply a general single-qubit Kraus map to a density matrix.
    ///
    /// The map is specified by at most four Kraus operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let m = &ComplexMatrix2::new([[0., 1.], [1., 0.]], [[0., 0.], [0., 0.]]);
    /// let target = 1;
    /// qureg.mix_kraus_map(target, &[m]).unwrap();
    ///
    /// // Check is the register is now in the state |01>
    /// let amp = qureg.get_density_amp(2, 2).unwrap();
    /// assert!((amp.re - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_kraus_map(
        &mut self,
        target: i32,
        ops: &[&ComplexMatrix2],
    ) -> Result<(), QuestError> {
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixKrausMap(self.reg, target, ops_inner.as_ptr(), num_ops);
        })
    }

    /// Apply a general two-qubit Kraus map to a density matrix.
    ///
    /// The map is specified by at most sixteen Kraus operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let m = &ComplexMatrix4::new(
    ///     [
    ///         [0., 0., 0., 1.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [1., 0., 0., 0.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    /// let target1 = 1;
    /// let target2 = 2;
    /// qureg
    ///     .mix_two_qubit_kraus_map(target1, target2, &[m])
    ///     .unwrap();
    ///
    /// // Check is the register is now in the state |011>
    /// let amp = qureg.get_density_amp(6, 6).unwrap();
    /// assert!((amp.re - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_two_qubit_kraus_map(
        &mut self,
        target1: i32,
        target2: i32,
        ops: &[&ComplexMatrix4],
    ) -> Result<(), QuestError> {
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixTwoQubitKrausMap(
                self.reg,
                target1,
                target2,
                ops_inner.as_ptr(),
                num_ops,
            );
        })
    }

    /// Apply a general N-qubit Kraus map to a density matrix.
    ///
    /// The map is specified by at most `(2N)^2` Kraus operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let m = &mut ComplexMatrixN::try_new(2).unwrap();
    /// init_complex_matrix_n(
    ///     m,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[1., 0., 0., 0.],
    ///     ],
    ///     &[
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///     ],
    /// )
    /// .unwrap();
    /// let targets = &[1, 2];
    /// qureg.mix_multi_qubit_kraus_map(targets, &[m]).unwrap();
    ///
    /// // Check if the register is now in the state |011>
    /// let amp = qureg.get_density_amp(6, 6).unwrap();
    /// assert!((amp.re - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_multi_qubit_kraus_map(
        &mut self,
        targets: &[i32],
        ops: &[&ComplexMatrixN],
    ) -> Result<(), QuestError> {
        let num_targets = targets.len() as i32;
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixMultiQubitKrausMap(
                self.reg,
                targets.as_ptr(),
                num_targets,
                ops_inner.as_ptr(),
                num_ops,
            );
        })
    }

    /// Apply a general non-trace-preserving single-qubit Kraus map.
    ///
    /// The state must be a density matrix, and the map is specified by at most
    /// four operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(2, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let m = &ComplexMatrix2::new([[0., 1.], [0., 0.]], [[0., 0.], [0., 0.]]);
    /// let target = 1;
    /// qureg.mix_nontp_kraus_map(target, &[m]).unwrap();
    ///
    /// // The register is in an unphysical null state
    /// let amp = qureg.get_density_amp(2, 2).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_nontp_kraus_map(
        &mut self,
        target: i32,
        ops: &[&ComplexMatrix2],
    ) -> Result<(), QuestError> {
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixNonTPKrausMap(
                self.reg,
                target,
                ops_inner.as_ptr(),
                num_ops,
            );
        })
    }

    /// Apply a general non-trace-preserving two-qubit Kraus map.
    ///
    /// The state must be a density matrix, and the map is specified
    /// by at most 16 operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    ///
    /// let m = &ComplexMatrix4::new(
    ///     [
    ///         [0., 0., 0., 1.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    /// let target1 = 1;
    /// let target2 = 2;
    /// qureg
    ///     .mix_nontp_two_qubit_kraus_map(target1, target2, &[m])
    ///     .unwrap();
    ///
    /// // The register is in an unphysical null state
    /// let amp = qureg.get_density_amp(6, 6).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_nontp_two_qubit_kraus_map(
        &mut self,
        target1: i32,
        target2: i32,
        ops: &[&ComplexMatrix4],
    ) -> Result<(), QuestError> {
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixNonTPTwoQubitKrausMap(
                self.reg,
                target1,
                target2,
                ops_inner.as_ptr(),
                num_ops,
            );
        })
    }

    /// Apply a general N-qubit non-trace-preserving Kraus map.
    ///
    /// The state must be a density matrix, and the map is specified
    /// by at most `2^(2N)` operators.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg = Qureg::try_new_density(3, &env)
    ///     .expect("cannot allocate memory for Qureg");
    /// let m = &mut ComplexMatrixN::try_new(2).unwrap();
    /// init_complex_matrix_n(
    ///     m,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[0., 0., 0., 0.],
    ///     ],
    ///     &[
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///         &[0., 0., 0., 0.],
    ///     ],
    /// )
    /// .unwrap();
    /// let targets = &[1, 2];
    /// qureg
    ///     .mix_nontp_multi_qubit_kraus_map(targets, &[m])
    ///     .unwrap();
    ///
    /// // The register is in an unphysical null state
    /// let amp = qureg.get_density_amp(6, 6).unwrap();
    /// assert!(amp.re.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn mix_nontp_multi_qubit_kraus_map(
        &mut self,
        targets: &[i32],
        ops: &[&ComplexMatrixN],
    ) -> Result<(), QuestError> {
        let num_targets = targets.len() as i32;
        let num_ops = ops.len() as i32;
        let ops_inner = ops.iter().map(|x| x.0).collect::<Vec<_>>();
        catch_quest_exception(|| unsafe {
            ffi::mixNonTPMultiQubitKrausMap(
                self.reg,
                targets.as_ptr(),
                num_targets,
                ops_inner.as_ptr(),
                num_ops,
            );
        })
    }

    /// Applies a trotterisation of unitary evolution.
    ///
    /// The unitary evelution `$\exp(-i \, \text{hamil} \, \text{time})$` is
    /// applied to `qureg`. # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// use PauliOpType::PAULI_X;
    ///
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(1, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let hamil = &mut PauliHamil::try_new(1, 1).unwrap();
    /// let coeffs = &[1.];
    /// let codes = &[PAULI_X];
    /// init_pauli_hamil(hamil, coeffs, codes).unwrap();
    ///
    /// let time = PI / 2.;
    /// let order = 1;
    /// let reps = 1;
    /// qureg
    ///     .apply_trotter_circuit(hamil, time, order, reps)
    ///     .unwrap();
    ///
    /// // qureg is now in `|1>` state:
    /// let qb1 = qureg.measure(0).unwrap();
    /// assert_eq!(qb1, 1);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_trotter_circuit(
        &mut self,
        hamil: &PauliHamil,
        time: Qreal,
        order: i32,
        reps: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::applyTrotterCircuit(self.reg, hamil.0, time, order, reps);
        })
    }

    /// Apply a general 2-by-2 matrix, which may be non-unitary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let target_qubit = 0;
    /// let u = &ComplexMatrix2::new([[0., 1.], [1., 0.]], [[0., 0.], [0., 0.]]);
    /// qureg.apply_matrix2(target_qubit, u).unwrap();
    ///
    /// let amp = qureg.get_real_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_matrix2(
        &mut self,
        target_qubit: i32,
        u: &ComplexMatrix2,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::applyMatrix2(self.reg, target_qubit, u.0);
        })
    }

    /// Apply a general 4-by-4 matrix, which may be non-unitary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let target_qubit1 = 0;
    /// let target_qubit2 = 1;
    /// let u = &ComplexMatrix4::new(
    ///     [
    ///         [0., 1., 0., 0.],
    ///         [1., 0., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.],
    ///     ],
    ///     [
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///         [0., 0., 0., 0.],
    ///     ],
    /// );
    ///
    /// qureg
    ///     .apply_matrix4(target_qubit1, target_qubit2, u)
    ///     .unwrap();
    ///
    /// let amp = qureg.get_real_amp(1).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_matrix4(
        &mut self,
        target_qubit1: i32,
        target_qubit2: i32,
        u: &ComplexMatrix4,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::applyMatrix4(self.reg, target_qubit1, target_qubit2, u.0);
        })
    }

    /// Apply a general N-by-N matrix on any number of target qubits.
    ///
    /// The matrix need not be unitary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let mtr = &mut ComplexMatrixN::try_new(3).unwrap();
    /// let empty = &[0., 0., 0., 0., 0., 0., 0., 0.];
    /// init_complex_matrix_n(
    ///     mtr,
    ///     &[
    ///         &[0., 0., 0., 0., 0., 0., 0., 1.],
    ///         &[0., 1., 0., 0., 0., 0., 0., 0.],
    ///         &[0., 0., 1., 0., 0., 0., 0., 0.],
    ///         &[0., 0., 0., 1., 0., 0., 0., 0.],
    ///         &[0., 0., 0., 0., 1., 0., 0., 0.],
    ///         &[0., 0., 0., 0., 0., 1., 0., 0.],
    ///         &[0., 0., 0., 0., 0., 0., 1., 0.],
    ///         &[1., 0., 0., 0., 0., 0., 0., 0.],
    ///     ],
    ///     &[empty, empty, empty, empty, empty, empty, empty, empty],
    /// )
    /// .unwrap();
    ///
    /// let targets = &[0, 1, 2];
    /// qureg.apply_matrix_n(targets, mtr).unwrap();
    ///
    /// // Check if the state is now `|111>`
    /// let amp = qureg.get_real_amp(7).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_matrix_n(
        &mut self,
        targs: &[i32],
        u: &ComplexMatrixN,
    ) -> Result<(), QuestError> {
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyMatrixN(self.reg, targs.as_ptr(), num_targs, u.0);
        })
    }

    /// Apply a general N-by-N matrix with additional controlled qubits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(4, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(0).unwrap();
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let ctrls = &[0, 1];
    /// let targs = &[2, 3];
    /// let u = &mut ComplexMatrixN::try_new(2).unwrap();
    /// let zero_row = &[0., 0., 0., 0.];
    /// init_complex_matrix_n(
    ///     u,
    ///     &[
    ///         &[0., 0., 0., 1.],
    ///         &[0., 1., 0., 0.],
    ///         &[0., 0., 1., 0.],
    ///         &[1., 0., 0., 0.],
    ///     ],
    ///     &[zero_row, zero_row, zero_row, zero_row],
    /// )
    /// .unwrap();
    /// qureg
    ///     .apply_multi_controlled_matrix_n(ctrls, targs, u)
    ///     .unwrap();
    ///
    /// // Assert `qureg` is now in the state `|1111>`
    /// let amp = qureg.get_real_amp(15).unwrap();
    /// assert!((amp - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_multi_controlled_matrix_n(
        &mut self,
        ctrls: &[i32],
        targs: &[i32],
        u: &ComplexMatrixN,
    ) -> Result<(), QuestError> {
        let num_ctrls = ctrls.len() as i32;
        let num_targs = targs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyMultiControlledMatrixN(
                self.reg,
                ctrls.as_ptr(),
                num_ctrls,
                targs.as_ptr(),
                num_targs,
                u.0,
            );
        })
    }

    /// Apply a phase function.
    ///
    /// Induces a phase change upon each amplitude of `qureg`, determined by the
    /// passed exponential polynomial *phase function*.  This effects a diagonal
    /// unitary of unit complex scalars, targeting the nominated `qubits`.
    ///
    /// - Arguments `coeffs` and `exponents` together specify a real exponential
    ///   polynomial `f(r)` with `num_terms` terms, of the form
    ///  
    ///   ```latex
    ///   f(r) =
    ///     \sum\limits_{i}^{\text{num_terms}} \text{coeffs}[i] \;
    ///     r^{\, \text{exponents}[i]}\,, \f],
    ///   ```
    ///
    ///   where both `coeffs` and `exponents` can be negative, positive and
    ///   fractional. For example,
    ///  
    ///   ```rust,no_run
    ///   let coeffs = [1., -3.14];
    ///   let exponents = [2., -5.5];
    ///   ```
    ///  
    ///   constitutes the function: `f(r) =  1 * r^2 - 3.14 * r^(-5.5)`.  Note
    ///   that you cannot use fractional exponents with `encoding` being
    ///   [`BitEncoding::TWOS_COMPLEMENT`],  since the
    ///   negative   indices would generate (illegal) complex phases, and  must
    /// be   overriden with
    ///   [`apply_phase_func_overrides()`].  
    ///  
    ///   If your function `f(r)` diverges at one or more `r` values, you
    ///   must instead use `apply_phase_func_overrides()` and specify explicit
    /// phase   changes for these values. Otherwise, the corresponding
    /// amplitudes of the   state-vector will become indeterminate (like
    /// `NaN`). Note that use of any   negative exponent will result in
    /// divergences at `r=0`.
    ///
    /// - The function `f(r)` specifies the phase change to induce upon
    ///   amplitude `alpha` of computational basis state with index `r`, such
    ///   that
    ///
    ///   ```latex
    ///   \alpha |r\rangle \rightarrow \, \exp(i f(r))  \alpha \,  |r\rangle.
    ///   ```
    ///
    ///   The index `r` associated with each computational basis
    ///   state is determined by the binary value of the specified `qubits`
    ///   (ordered least to most significant), interpreted under the given
    ///   [`BitEncoding`] encoding.
    ///
    /// - If `qureg` is a density matrix `rho`, this function modifies `qureg`
    ///   to:
    ///
    ///   ```latex
    ///   \rho \rightarrow \hat{D} \, \rho \, \hat{D}^\dagger,
    ///   ```
    ///
    ///   where   `\hat{D}` is the diagonal unitary operator:
    ///
    ///   ```latex
    ///    \hat{D} = \text{diag}
    ///     \, \{ \; e^{i f(r_0)}, \; e^{i f(r_1)}, \;  \dots \; \}.
    ///   ```
    ///
    /// - The interpreted phase function can be previewed in the QASM log, as a
    ///   comment.
    ///
    /// - This function may become numerically imprecise for quickly growing
    ///   phase functions which admit very large phases, for example of `10^10`.
    ///
    /// # Parameters
    ///
    /// - `qubits`: a list of the indices of the qubits which will inform `r`
    ///   for each amplitude in `qureg`
    /// - `encoding`: the [`BitEncoding`] under which to infer the binary value
    ///   `r` from the bits of `qubits` in each basis state of `qureg`
    /// - `coeffs`: the coefficients of the exponential polynomial phase
    ///   function `f(r)`
    /// - `exponents`: the exponents of the exponential polynomial phase
    ///   function `f(r)`
    ///
    /// The length of list `coeffs` must be the same as that of `exponents`
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`]
    ///   - if the length of `coeffs` is different than that of  `exponents`
    ///   - if any qubit in `qubits` has an invalid index (i.e. does not satisfy
    ///     `0 <= qubit < qureg.num_qubits()`
    ///   - if the elements of `qubits` are not unique
    ///   - if `qubits.len() >= qureg.num_qubits()`
    ///   - if `encoding` is not compatible with `qubits.len()` (e.g.
    ///     `TWOS_COMPLEMENT` with only 1 qubit)
    ///   - if `exponents` contains a fractional number despite `encoding` being
    ///     `TWOS_COMPLEMENT` (you must instead use
    ///     `apply_phase_func_overrides()` and override all negative indices)
    ///   - if `exponents` contains a negative power (you must instead use
    ///     apply_phase_func_overrides()` and override the zero index)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let qubits = &[0, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let coeffs = &[0.5, 0.5];
    /// let exponents = &[0., 2.];
    ///
    /// qureg
    ///     .apply_phase_func(qubits, encoding, coeffs, exponents)
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`BitEncoding::TWOS_COMPLEMENT`]: crate::BitEncoding::TWOS_COMPLEMENT
    /// [`BitEncoding`]: crate::BitEncoding
    /// [`apply_phase_func_overrides()`]: crate::Qureg::apply_phase_func_overrides()
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_phase_func(
        &mut self,
        qubits: &[i32],
        encoding: BitEncoding,
        coeffs: &[Qreal],
        exponents: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_qubits = qubits.len() as i32;
        let num_terms = coeffs.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyPhaseFunc(
                self.reg,
                qubits.as_ptr(),
                num_qubits,
                encoding,
                coeffs.as_ptr(),
                exponents.as_ptr(),
                num_terms,
            );
        })
    }

    /// Apply a phase function with overrides.
    ///
    /// Induces a phase change upon each amplitude of `qureg`, determined by the
    /// passed  exponential polynomial "phase function", and an explicit set of
    /// 'overriding' values at specific state indices.
    ///
    /// See [`apply_phase_func()`] for a full desctiption.
    ///
    /// - As in `apply_phase_func()`, the arguments `coeffs` and `exponents`
    ///   specify a phase function `f(r)`, where `r` is determined by `qubits`
    ///   and `encoding` for each basis state of `qureg`.
    /// - Additionally, `override_inds` is a list specifying the values of `r`
    ///   for which to explicitly set the induced phase change. The overriding
    ///   phase changes are specified in the corresponding elements of
    ///   `override_phases`.
    /// - Note that if `encoding` is `TWOS_COMPLEMENT`, and `f(r)` features a
    ///   fractional exponent, then every negative phase index must be
    ///   overriden. This is checked and enforced by `QuEST`'s validation,
    ///   unless there are more than 16 targeted qubits, in which case valid
    ///   input is assumed (due to an otherwise prohibitive performance
    ///   overhead).
    /// - Overriding phases are checked at each computational basis state of
    ///   `qureg` *before* evaluating the phase function `f(r)`, and hence are
    ///   useful for avoiding singularities or errors at diverging values of
    ///   `r`.
    /// - The interpreted phase function and list of overrides can be previewed
    ///   in the QASM log, as a comment.
    ///
    /// # Parameters
    ///
    /// - `qubits`: a list of the indices of the qubits which will inform `r`
    ///   for each amplitude in `qureg`
    /// - `encoding`: [`BitEncoding`] under which to infer the binary value `r`
    ///   from the bits of `qubits` in each basis state of `qureg`
    /// - `coeffs`: the coefficients of the exponential polynomial phase
    ///   function `f(r)`
    /// - `exponents`: the exponents of the exponential polynomial phase
    ///   function `f(r)`
    /// - `override_inds`: a list of sub-state indices (values of `r` of which
    ///   to explicit set the phase change
    /// - `override_phases`: a list of replacement phase changes, for the
    ///   corresponding `r` values in `override_inds` (one to one)
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if the length of `override_inds` is different than that of
    ///     `override_phases`
    ///   - if the length of `coeffs` is different than that of `exponents`
    ///   - if any qubit in `qubits` has an invalid index (i.e. does not satisfy
    ///     `0 <= qubit < qureg.num_qubits()`
    ///   - if the elements of `qubits` are not unique
    ///   - if `qubits.len() >= qureg.num_qubits()`
    ///   - if `encoding` is not compatible with `qubits.len()` (e.g.
    ///     `TWOS_COMPLEMENT` with only 1 qubit)
    ///   - if `exponents` contains a fractional number despite `encoding` being
    ///     `TWOS_COMPLEMENT` (you must instead use
    ///     `apply_phase_func_overrides()` and override all negative indices)
    ///   - if `exponents` contains a negative power and the (consequently
    ///     diverging) zero index is not contained in `override_inds`
    ///   - if any value in `override_inds` is not producible by `qubits` under
    ///     the given `encoding` (e.g. 2 unsigned qubits cannot represent index
    ///     9)
    ///   - if `encoding` is `TWOS_COMPLEMENT`, and `exponents` contains a
    ///     fractional number, but `override_inds` does not contain every
    ///     possible negative index (checked only up to 16 targeted qubits)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let qubits = &[0, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let coeffs = &[0.5, 0.5];
    /// let exponents = &[-2., 2.];
    /// let override_inds = &[0];
    /// let override_phases = &[0.];
    ///
    /// qureg
    ///     .apply_phase_func_overrides(
    ///         qubits,
    ///         encoding,
    ///         coeffs,
    ///         exponents,
    ///         override_inds,
    ///         override_phases,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`apply_phase_func()`]: crate::Qureg::apply_phase_func()
    /// [`BitEncoding`]: crate::BitEncoding
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_phase_func_overrides(
        &mut self,
        qubits: &[i32],
        encoding: BitEncoding,
        coeffs: &[Qreal],
        exponents: &[Qreal],
        override_inds: &[i64],
        override_phases: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_qubits = qubits.len() as i32;
        let num_terms = coeffs.len() as i32;
        let num_overrides = override_inds.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyPhaseFuncOverrides(
                self.reg,
                qubits.as_ptr(),
                num_qubits,
                encoding,
                coeffs.as_ptr(),
                exponents.as_ptr(),
                num_terms,
                override_inds.as_ptr(),
                override_phases.as_ptr(),
                num_overrides,
            );
        })
    }

    /// Apply a multi-variable exponential polynomial.
    ///
    /// Induces a phase change upon each amplitude of `qureg`, determined by the
    /// multi-variable exponential polynomial "phase function".
    ///
    /// This is a multi-variable extension of [`apply_phase_func()`], whereby
    /// multiple sub-registers inform separate variables in the exponential
    /// polynomial function, and effects a diagonal unitary operator.
    ///
    /// - Arguments `coeffs`, `exponents` and `num_terms_per_reg` together
    ///   specify a real exponential polynomial `f(r)` of the form
    ///
    ///   ```latex
    ///    f(r_1,\dots, \; r_{\text{numRegs}}) =
    ///    \sum\limits_j^{\text{numRegs}}
    ///   \sum\limits_{i}^{\text{numTermsPerReg}[j]} c_{i,j} \; {r_j}^{p_{i,j}},
    ///   ```
    ///
    ///   where both coefficients `c_{i,j}` and exponents `p_{i,j}` can be any
    /// real   number, subject to constraints described below.
    ///  
    ///   While `coeffs` and `exponents` are flat lists, they should be
    /// considered   grouped into `num_qubits_per_reg.len()` sublists with
    /// lengths given by   `num_qubits_per_reg`.
    ///
    ///   For example,
    ///
    ///   ```rust,no_run
    ///   let coeffs =            [1., 2., 4., -3.14];
    ///   let exponents =         [2., 1., 5., 0.5];
    ///   let num_terms_per_reg = [1., 2.,     1.];
    ///   ```
    ///
    ///   constitutes the function: `f(\vec{r}) =  1 * {r_1}^2 + 2 * {r_2} + 4
    /// \,   {r_2}^{5} - 3.14 \, {r_3}^{0.5}`.   This means lists `coeffs`
    /// and   `exponents` should both be of length   equal to the sum of
    ///   `num_terms_per_reg`.
    ///
    ///
    /// - Unlike [`apply_phase_func()`], this function places additional
    ///   constraints on the   exponents in `f(\vec{r})`, due to the
    ///   exponentially growing costs of overriding diverging indices. Namely:
    ///
    ///   - `exponents` must not contain a negative number, since this would
    ///     result in a divergence when that register is zero, which would need
    ///     to be overriden for every other register basis state.  If
    ///     `f(\vec{r})` must contain a negative exponent, you should instead
    ///     call [`apply_phase_func_overrides()`] once for each
    ///     register/variable, and override the zero index for the relevant
    ///     variable. This works, because `\exp( i \sum_j f_j(r_j) ) = \prod_j
    ///     \exp(i f_j(r_j) )`.
    ///   - `exponents` must not contain a fractional number if `endoding =
    ///     TWOS_COMPLEMENT`, because such a term would produce illegal complex
    ///     values at negative register indices. Similar to the problem above,
    ///     each negative register index would require overriding at every index
    ///     of the other registers, and hence require an exponential number of
    ///     overrides. Therefore, if `f(\vec{r})` must contain a negative
    ///     exponent, you should instead call `apply_phase_func_overrides()`
    ///     once for each register/variable, and override every negative index
    ///     of each register in turn.
    ///
    /// - Lists `qubits` and `num_qubits_per_reg` together describe
    ///   sub-registers of `qureg`, which can each contain a different number of
    ///   qubits. Although `qubits` is a flat list of unique qubit indices, it
    ///   should be imagined grouped into sub-lists, of lengths given by
    ///   `num_qubits_per_reg`.
    ///
    ///   Note that the qubits need not be ordered increasing, and
    ///   qubits within each sub-register are assumed ordered least to most
    ///   significant in that sub-register. List `qubits` should have length
    /// equal   to the sum of elements in `num_qubits_per_reg`.
    ///
    /// - Each sub-register is associated with a variable `r_j` in phase
    ///   function `f(\vec{r})`. For a given computational basis state of
    ///   `qureg`, the value of each variable is determined by the binary value
    ///   in the corresponding sub-register, when intepreted with
    ///   [`BitEncoding`] `encoding`.
    ///
    /// - The function `f(\vec{r})` specifies the phase change to induce upon
    ///   amplitude `alpha` of computational basis state with the nominated
    ///   sub-registers encoding values.
    ///
    /// - The interpreted phase function can be previewed in the QASM log, as a
    ///   comment.
    ///
    /// # Parameters
    ///
    /// - `qubits`: a list of all the qubit indices contained in each
    ///   sub-register
    /// - `num_qubits_per_reg`: a list of the lengths of each sub-list in
    ///   `qubits`
    /// - `encoding`: [`BitEncoding`] under which to infer the binary value
    ///   `r_j` from the bits of a sub-register
    /// - `coeffs`: the coefficients of all terms of the exponential polynomial
    ///   phase function `f(\vec{r})`
    /// - `exponents`: the exponents of all terms of the exponential polynomial
    ///   phase function `f(\vec{r})`
    /// - `num_terms_per_reg` a list of the number of `coeff` and `exponent`
    ///   terms supplied for each variable/sub-register
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if any qubit in `qubits` has an invalid index (i.e. does not satisfy
    ///     0 <= qubit < `qureg.num_qubits()`)
    ///   - if the elements of `qubits` are not unique (including if
    ///     sub-registers overlap)
    ///   - if `num_qubits_per_reg.len() = 0 or > 100` (constrained by
    ///     `MAX_NUM_REGS_APPLY_ARBITRARY_PHASE` in `QuEST_precision.h`)
    ///   - if the size of any sub-register is incompatible with `encoding`
    ///     (e.g. contains fewer than two qubits if `encoding =
    ///     TWOS_COMPLEMENT`)
    ///   - if any element of `num_terms_per_reg < 1`
    ///   - if `exponents` contains a negative number
    ///   - if `exponents` contains a fractional number despite `encoding =
    ///     TWOS_COMPLEMENT`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let coeffs = &[0.5, 0.5];
    /// let exponents = &[2., 2.];
    /// let num_terms_per_reg = &[1, 1];
    ///
    /// qureg
    ///     .apply_multi_var_phase_func(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         coeffs,
    ///         exponents,
    ///         num_terms_per_reg,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`apply_phase_func()`]: crate::Qureg::apply_phase_func()
    /// [`apply_phase_func_overrides()`]: crate::Qureg::apply_phase_func_overrides()
    /// [`BitEncoding`]: crate::BitEncoding
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_multi_var_phase_func(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        coeffs: &[Qreal],
        exponents: &[Qreal],
        num_terms_per_reg: &[i32],
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyMultiVarPhaseFunc(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                coeffs.as_ptr(),
                exponents.as_ptr(),
                num_terms_per_reg.as_ptr(),
            );
        })
    }

    /// Apply a multi-variable exponential polynomial with overrides.
    ///
    /// Induces a phase change upon each amplitude of `qureg`, determined by a
    /// phase function, and an explicit set of 'overriding' values at specific
    /// state indices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    /// qureg.pauli_x(1).unwrap();
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let coeffs = &[0.5, 0.5];
    /// let exponents = &[2., 2.];
    /// let num_terms_per_reg = &[1, 1];
    /// let override_inds = &[0, 1, 0, 1];
    /// let override_phases = &[0., 0.];
    ///
    /// qureg
    ///     .apply_multi_var_phase_func_overrides(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         coeffs,
    ///         exponents,
    ///         num_terms_per_reg,
    ///         override_inds,
    ///         override_phases,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_multi_var_phase_func_overrides(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        coeffs: &[Qreal],
        exponents: &[Qreal],
        num_terms_per_reg: &[i32],
        override_inds: &[i64],
        override_phases: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        let num_overrides = override_phases.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyMultiVarPhaseFuncOverrides(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                coeffs.as_ptr(),
                exponents.as_ptr(),
                num_terms_per_reg.as_ptr(),
                override_inds.as_ptr(),
                override_phases.as_ptr(),
                num_overrides,
            );
        })
    }

    /// Apply a named phase function.
    ///
    /// Induces a phase change upon each amplitude of `qureg`, determined by a
    /// named (and potentially multi-variable) phase function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let function_name_code = PhaseFunc::DISTANCE;
    ///
    /// qureg
    ///     .apply_named_phase_func(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         function_name_code,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_named_phase_func(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        function_name_code: PhaseFunc,
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyNamedPhaseFunc(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                function_name_code,
            );
        })
    }

    /// Apply a named phase function with overrides.
    ///
    /// Induces a phase change upon each amplitude of \p qureg, determined by a
    /// named (and potentially multi-variable) phase function, and an explicit
    /// set of 'overriding' values at specific state indices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let function_name_code = PhaseFunc::DISTANCE;
    /// let override_inds = &[0, 1, 0, 1];
    /// let override_phases = &[0., 0.];
    ///
    /// qureg
    ///     .apply_named_phase_func_overrides(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         function_name_code,
    ///         override_inds,
    ///         override_phases,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_named_phase_func_overrides(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        function_name_code: PhaseFunc,
        override_inds: &[i64],
        override_phases: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        let num_overrides = override_phases.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyNamedPhaseFuncOverrides(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                function_name_code,
                override_inds.as_ptr(),
                override_phases.as_ptr(),
                num_overrides,
            );
        })
    }

    /// Apply a parametrized phase function.
    ///
    /// Induces a phase change upon each amplitude of \p qureg, determined by a
    /// named, paramaterized (and potentially multi-variable) phase function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let function_name_code = PhaseFunc::SCALED_INVERSE_SHIFTED_NORM;
    /// let params = &[0., 0., 0., 0.];
    ///
    /// qureg
    ///     .apply_param_named_phase_func(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         function_name_code,
    ///         params,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_param_named_phase_func(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        function_name_code: PhaseFunc,
        params: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        let num_params = params.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyParamNamedPhaseFunc(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                function_name_code,
                params.as_ptr(),
                num_params,
            );
        })
    }

    /// Apply a parametrized phase function with overrides.
    ///
    /// Induces a phase change upon each amplitude of \p qureg, determined by a
    /// named, parameterised (and potentially multi-variable) phase function,
    /// and an explicit set of "overriding" values at specific state
    /// indices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// let qubits = &[0, 1];
    /// let num_qubits_per_reg = &[1, 1];
    /// let encoding = BitEncoding::UNSIGNED;
    /// let function_name_code = PhaseFunc::SCALED_INVERSE_SHIFTED_NORM;
    /// let params = &[0., 0., 0., 0.];
    /// let override_inds = &[0, 1, 0, 1];
    /// let override_phases = &[0., 0.];
    ///
    /// qureg
    ///     .apply_param_named_phase_func_overrides(
    ///         qubits,
    ///         num_qubits_per_reg,
    ///         encoding,
    ///         function_name_code,
    ///         params,
    ///         override_inds,
    ///         override_phases,
    ///     )
    ///     .unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_param_named_phase_func_overrides(
        &mut self,
        qubits: &[i32],
        num_qubits_per_reg: &[i32],
        encoding: BitEncoding,
        function_name_code: PhaseFunc,
        params: &[Qreal],
        override_inds: &[i64],
        override_phases: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_regs = num_qubits_per_reg.len() as i32;
        let num_params = params.len() as i32;
        let num_overrides = override_phases.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyParamNamedPhaseFuncOverrides(
                self.reg,
                qubits.as_ptr(),
                num_qubits_per_reg.as_ptr(),
                num_regs,
                encoding,
                function_name_code,
                params.as_ptr(),
                num_params,
                override_inds.as_ptr(),
                override_phases.as_ptr(),
                num_overrides,
            );
        })
    }

    /// Apply the full quantum Fourier transform (QFT).
    ///
    /// - If `qureg` is a state-vector, the output amplitudes are the discrete
    ///   Fourier transform (DFT) of the input amplitudes, in the exact
    ///   ordering. This is true even if `qureg` is unnormalised.
    ///
    /// - If `qureg` is a density matrix, it will be changed under the unitary
    ///   action of the QFT. This can be imagined as each mixed state-vector
    ///   undergoing the DFT on its amplitudes. This is true even if `qureg` is
    ///   unnormalised.
    ///
    /// This function merges contiguous controlled-phase gates into single
    /// invocations of [`apply_named_phase_func`()][api-apply-named-phase-func],
    /// and hence is significantly faster than performing
    /// the QFT circuit directly.
    ///
    /// Furthermore, in distributed mode, this function requires only
    /// `log2(#nodes)` rounds of pair-wise communication, and hence is
    /// exponentially faster than directly performing the DFT on the
    /// amplitudes of `qureg`.
    ///
    /// See [`apply_qft()`][api-apply-qft] to apply the QFT to a sub-register of
    /// `qureg`.
    ///
    /// # Parameters
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.apply_full_qft();
    /// ```
    /// See [QuEST API] for more information.
    ///
    /// [api-apply-named-phase-func]: crate::Qureg::apply_named_phase_func()
    /// [api-apply-qft]: crate::Qureg::apply_qft()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_full_qft(&mut self) {
        catch_quest_exception(|| unsafe {
            ffi::applyFullQFT(self.reg);
        })
        .expect("apply_full_qft should always succeed");
    }

    /// Applies the quantum Fourier transform (QFT) to a specific subset of
    /// qubits.
    ///
    /// The order of qubits affects the ultimate unitary.
    /// The canonical full-state QFT ([`apply_full_qft()`]) is
    /// achieved by targeting every qubit in increasing order.
    ///
    /// - If `qureg` is a state-vector, the output amplitudes are a kronecker
    ///   product of the discrete Fourier transform (DFT) acting upon the
    ///   targeted amplitudes.
    /// - If `qureg` is a density matrix, it will be changed under the unitary
    ///   action of the QFT. This can be imagined as each mixed state-vector
    ///   undergoing the DFT on its amplitudes. This is true even if `qureg` is
    ///   unnormalised.
    ///
    /// This function merges contiguous controlled-phase gates into single
    /// invocations of [`apply_named_phase_func()`], and
    /// hence is significantly faster than performing
    /// the QFT circuit directly.
    ///
    ///
    /// Furthermore, in distributed mode, this function requires only
    /// `log2(#nodes)` rounds of pair-wise communication, and hence is
    /// exponentially faster than directly performing the DFT on the
    /// amplitudes of `qureg`.
    ///
    /// See [`apply_full_qft()`] to apply the QFT to he entirety
    /// of `Qureg`.
    ///
    /// # Parameters
    ///
    /// `qureg`: a state-vector or density matrix to modify
    /// `qubits` a list of the qubits to operate the QFT upon
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if the length of `qubits` is less than [`qureg.num_qubits()`]
    ///   - if any of `qubits` is outside [0, [`num_qubits()`]).
    ///   - if `qubits` contains any repetitions
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(3, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.apply_qft(&[0, 1]).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`apply_full_qft()`]: crate::Qureg::apply_full_qft()
    /// [`apply_named_phase_func()`]: crate::Qureg::apply_named_phase_func()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_qft(
        &mut self,
        qubits: &[i32],
    ) -> Result<(), QuestError> {
        let num_qubits = qubits.len() as i32;
        catch_quest_exception(|| unsafe {
            ffi::applyQFT(self.reg, qubits.as_ptr(), num_qubits);
        })
    }

    /// Apply a projector.
    ///
    /// Force the target `qubit` of `qureg` into the given classical `outcome`,
    /// via a non-renormalising projection.
    ///
    /// This function zeroes all amplitudes in the state-vector or
    /// density-matrix which correspond to the opposite `outcome` given.
    /// Unlike [`collapse_to_outcome()`], it does not thereafter normalise
    /// `qureg`, and hence may leave it in a non-physical state.
    ///
    /// Note there is no requirement that the `outcome` state has a non-zero
    /// proability, and hence this function may leave `qureg` in a blank state,
    /// like that produced by [`init_blank_state()`].
    ///
    /// See [`collapse_to_outcome()`] for a norm-preserving equivalent, like a
    /// forced measurement
    ///
    /// # Parameters
    ///
    /// - `qubit`: the qubit to which to apply the projector
    /// - `outcome`: the single-qubit outcome (`0` or `1`) to project `qubit`
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qubit` is outside [0, [`num_qubits()`]).
    ///   - if `outcome` is not in {0,1}
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = QuestEnv::new();
    /// let mut qureg =
    ///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
    ///
    /// qureg.apply_projector(0, 0).unwrap();
    ///
    /// let amp = qureg.get_real_amp(3).unwrap();
    /// assert!(amp.abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`collapse_to_outcome()`]: crate::Qureg::collapse_to_outcome()
    /// [`init_blank_state()`]: crate::Qureg::init_blank_state()
    /// [`QubitIndexError`]: crate::QuestError::QubitIndexError
    /// [`num_qubits()`]: crate::Qureg::num_qubits()
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn apply_projector(
        &mut self,
        qubit: i32,
        outcome: i32,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::applyProjector(self.reg, qubit, outcome);
        })
    }
} // Qureg

impl<'a> Drop for Qureg<'a> {
    fn drop(&mut self) {
        catch_quest_exception(|| {
            unsafe { ffi::destroyQureg(self.reg, self.env.0) };
        })
        .expect("dropping Qureg should always succeed");
    }
}

/// Apply Hamiltonian `PauliHamil`.
///
/// Modifies `out_qureg` to be the result of applying `PauliHamil` (a
/// Hermitian but not necessarily unitary operator) to `in_qureg`.
///
/// In theory, `in_qureg` is unchanged though its state is temporarily
/// modified and is reverted by re-applying Paulis (XX=YY=ZZ=I), so may
/// see a change by small numerical errors. The initial state in
/// `out_qureg` is not used.
///
/// # Examples
///
/// ```rust
/// # use quest_bind::*;
/// use PauliOpType::{
///     PAULI_I,
///     PAULI_X,
/// };
///
/// let env = QuestEnv::new();
/// let mut in_qureg =
///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
/// let mut out_qureg =
///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
///
/// let hamil = &mut PauliHamil::try_new(2, 2).unwrap();
/// let coeffs = &[SQRT_2.recip(), SQRT_2.recip()];
/// let codes = &[PAULI_I, PAULI_X, PAULI_X, PAULI_I];
/// init_pauli_hamil(hamil, coeffs, codes).unwrap();
///
/// apply_pauli_hamil(&mut in_qureg, hamil, &mut out_qureg).unwrap();
///
/// // out_qureg is now in `|01> + |10>` state:
/// let qb1 = out_qureg.measure(0).unwrap();
/// let qb2 = out_qureg.measure(1).unwrap();
/// assert!(qb1 != qb2);
/// ```
///
/// See [QuEST API] for more information.
///
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
#[allow(clippy::needless_pass_by_ref_mut)]
pub fn apply_pauli_hamil(
    in_qureg: &mut Qureg<'_>,
    hamil: &PauliHamil,
    out_qureg: &mut Qureg<'_>,
) -> Result<(), QuestError> {
    catch_quest_exception(|| unsafe {
        ffi::applyPauliHamil(in_qureg.reg, hamil.0, out_qureg.reg);
    })
}

/// Apply the weighted sum of Pauli products.
///
/// In theory, `in_qureg` is unchanged though its state is temporarily
/// modified and is reverted by re-applying Paulis (XX=YY=ZZ=I), so may
/// see a change by small numerical errors. The initial state in
/// `out_qureg` is not used.
///
/// # Examples
///
/// ```rust
/// # use quest_bind::*;
/// use PauliOpType::{
///     PAULI_I,
///     PAULI_X,
/// };
///
/// let env = QuestEnv::new();
/// let mut in_qureg =
///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
/// let mut out_qureg =
///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
/// let all_pauli_codes = &[PAULI_I, PAULI_X, PAULI_X, PAULI_I];
/// let term_coeffs = &[SQRT_2.recip(), SQRT_2.recip()];
///
/// apply_pauli_sum(
///     &mut in_qureg,
///     all_pauli_codes,
///     term_coeffs,
///     &mut out_qureg,
/// )
/// .unwrap();
///
/// // out_qureg is now in `|01> + |10>` state:
/// let qb1 = out_qureg.measure(0).unwrap();
/// let qb2 = out_qureg.measure(1).unwrap();
/// assert!(qb1 != qb2);
/// ```
///
/// See [QuEST API] for more information.
///
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
#[allow(clippy::needless_pass_by_ref_mut)]
pub fn apply_pauli_sum(
    in_qureg: &mut Qureg<'_>,
    all_pauli_codes: &[PauliOpType],
    term_coeffs: &[Qreal],
    out_qureg: &mut Qureg<'_>,
) -> Result<(), QuestError> {
    let num_sum_terms = term_coeffs.len() as i32;
    catch_quest_exception(|| unsafe {
        ffi::applyPauliSum(
            in_qureg.reg,
            all_pauli_codes.as_ptr(),
            term_coeffs.as_ptr(),
            num_sum_terms,
            out_qureg.reg,
        );
    })
}

/// Computes the Hilbert Schmidt distance between two density matrices.
///
/// Defined as the Frobenius norm of the difference between them.
///
/// This is equivalent to the square-root of the sum of the absolute value
/// squared of the element-differences of the matrices.
///
/// We caution this may differ by some definitions of the Hilbert Schmidt
/// distance by a square-root.
///
/// This function correctly returns the result of the above formulations even
/// when `a` and `b` are incorrectly normalised (i.e. are general matrices).
///
/// # Parameters
///
/// - `a`: a density matrix
/// - `b`: an equally-sized density matrix
///
/// # Errors
///  
/// - [`InvalidQuESTInputError`]
///   - if either `a` or `b` are not density matrices
///   - if `a` and `b` have mismatching dimension
///
/// # Examples
///
/// ```rust
/// # use quest_bind::*;
/// let env = QuestEnv::new();
/// let a = Qureg::try_new_density(2, &env)
///     .expect("cannot allocate memory for Qureg");
/// let b = {
///     let mut b = Qureg::try_new_density(2, &env)
///         .expect("cannot allocate memory for Qureg");
///     b.init_classical_state(1).unwrap();
///     b
/// };
///
/// let dist = calc_hilbert_schmidt_distance(&a, &b).unwrap();
/// assert!((dist - SQRT_2).abs() < EPSILON, "{:?}", dist);
/// ```
///
/// See [QuEST API] for more information.
///
/// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
pub fn calc_hilbert_schmidt_distance(
    a: &Qureg<'_>,
    b: &Qureg<'_>,
) -> Result<Qreal, QuestError> {
    catch_quest_exception(|| unsafe {
        ffi::calcHilbertSchmidtDistance(a.reg, b.reg)
    })
}

/// Computes the inner product of two equal-size state vectors.
///
/// Given by
///
/// ```latex
///  \langle \text{bra} | \text{ket} \rangle = \sum_i {\text{bra}_i}^* \; \times \; \text{ket}_i
/// ```
///
/// The same `Qureg` may be passed as both `bra` and `ket`,
/// though we recommend users check state-vector normalisation with
/// [`calc_total_prob()`] which employs Kahan summation for greater accuracy.
///
/// Neither state-vector is modified.
///
/// This function returns the correct inner product even if `bra` and `ket` are
/// not correctly normalised states.
///
/// # Parameters
///
/// - `bra`: bra `Qureg` to be the "bra" (i.e. have its values conjugate
///   transposed) in the inner product
/// - `ket`: qureg to be the "ket" in the inner product
///
/// # Errors
///  
/// - [`InvalidQuESTInputError`]
///   - if either `bra` and `ket` are not both state-vectors
///   - if `bra` and `ket` do not have equal dimensions
///
/// # Examples
///
/// ```rust
/// # use quest_bind::*;
/// let env = QuestEnv::new();
/// let qureg =
///     Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
/// let other_qureg = {
///     let mut other_qureg =
///         Qureg::try_new(2, &env).expect("cannot allocate memory for Qureg");
///     other_qureg.init_plus_state();
///     other_qureg
/// };
///
/// let prod = calc_inner_product(&qureg, &other_qureg).unwrap();
/// assert!((prod.re - 0.5).abs() < EPSILON);
/// assert!((prod.im).abs() < EPSILON);
/// ```
///
/// See [QuEST API] for more information.
///
/// [`calc_total_prob()`]: Qureg::calc_total_prob()
/// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
pub fn calc_inner_product(
    bra: &Qureg<'_>,
    ket: &Qureg<'_>,
) -> Result<Qcomplex, QuestError> {
    catch_quest_exception(|| unsafe { ffi::calcInnerProduct(bra.reg, ket.reg) })
        .map(Into::into)
}

/// Computes the Hilbert-Schmidt scalar product.
///
/// # Examples
///
/// ```rust
/// # use quest_bind::*;
/// let env = QuestEnv::new();
/// let qureg = Qureg::try_new_density(2, &env)
///     .expect("cannot allocate memory for Qureg");
/// let other_qureg = {
///     let mut other_qureg = Qureg::try_new_density(2, &env)
///         .expect("cannot allocate memory for Qureg");
///     other_qureg.init_plus_state();
///     other_qureg
/// };
///
/// let prod = calc_density_inner_product(&qureg, &other_qureg).unwrap();
/// assert!((prod - 0.25).abs() < EPSILON);
/// ```
///
/// See [QuEST API] for more information.
///
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
pub fn calc_density_inner_product(
    rho1: &Qureg<'_>,
    rho2: &Qureg<'_>,
) -> Result<Qreal, QuestError> {
    catch_quest_exception(|| unsafe {
        ffi::calcDensityInnerProduct(rho1.reg, rho2.reg)
    })
}

/// Set `qureg` to a weighted sum of states.
///
/// Modifies qureg `out` to the result of `$(\p facOut \p out + \p fac1 \p
/// qureg1 + \p fac2 \p qureg2)$`, imposing no constraints on normalisation.
///
/// Works for both state-vectors and density matrices. Note that afterward,
/// \p out may not longer be normalised and ergo no longer a valid
/// state-vector or density matrix. Users must therefore be careful
/// passing \p out to other `QuEST` functions which assume normalisation
/// in order to function correctly.
///
///
/// See [QuEST API] for more information.
///
/// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
#[allow(clippy::needless_pass_by_ref_mut)]
pub fn set_weighted_qureg(
    fac1: Qcomplex,
    qureg1: &Qureg<'_>,
    fac2: Qcomplex,
    qureg2: &Qureg<'_>,
    fac_out: Qcomplex,
    out: &mut Qureg<'_>,
) -> Result<(), QuestError> {
    catch_quest_exception(|| unsafe {
        ffi::setWeightedQureg(
            fac1.into(),
            qureg1.reg,
            fac2.into(),
            qureg2.reg,
            fac_out.into(),
            out.reg,
        );
    })
}
