use super::{
    catch_quest_exception,
    ffi,
    QuestEnv,
    QuestError,
};
use crate::Qreal;

#[derive(Debug)]
pub struct Qureg<'a, const N: u16> {
    pub(crate) env: &'a QuestEnv,
    pub(crate) reg: ffi::Qureg,
}

impl<'a, const N: u16> Qureg<'a, N> {
    /// Creates a state-vector Qureg object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = create_qureg::<2>(env);
    /// ```
    ///
    /// See [QuEST API][1] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`QuestError::InvalidQuESTInputError`](crate::QuestError::InvalidQuESTInputError)
    /// on failure.  This is an exception thrown by `QuEST`.
    ///
    /// [1]: https://quest-kit.github.io/QuEST/modules.html
    pub fn try_new(env: &'a QuestEnv) -> Result<Self, QuestError> {
        let num_qubits = N as i32;
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
    /// let env = &QuestEnv::new();
    /// let qureg = Qureg::try_new_density(2, env).unwrap();
    /// ```
    ///
    /// See [QuEST API][1] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`QuestError::InvalidQuESTInputError`](crate::QuestError::InvalidQuESTInputError)
    /// on failure.  This is an exception thrown by `QuEST`.
    ///
    /// [1]: https://quest-kit.github.io/QuEST/modules.html
    pub fn try_new_density(env: &'a QuestEnv) -> Result<Self, QuestError> {
        let num_qubits = N as i32;
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

    #[must_use]
    pub fn num_qubits_represented(&self) -> i32 {
        self.reg.numQubitsRepresented
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
    /// let env = &QuestEnv::new();
    /// let qureg = create_qureg::<3>(env);
    ///
    /// assert_eq!(qureg.get_num_qubits(), 3);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub fn get_num_qubits(&self) -> i32 {
        catch_quest_exception(|| unsafe { ffi::getNumQubits(self.reg) })
            .expect("get_num_qubits should never fail")
    }

    /// Return the number of complex amplitudes in a state-vector.
    ///
    /// In distributed mode, this returns the total number of amplitudes in the
    /// full representation of `qureg`, and so may be larger than the number
    /// stored on each node.
    ///
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`], if `Qureg` is a density matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &create_qureg::<3>(env);
    ///
    /// assert_eq!(qureg.get_num_amps().unwrap(), 8);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    pub fn get_num_amps(&self) -> Result<i64, QuestError> {
        catch_quest_exception(|| unsafe { ffi::getNumAmps(self.reg) })
    }

    /// Return the total number of amplitudes in the register.
    ///
    /// - If `Qureg` is a state-vector, this is equal to: `2^N`, where `N` is
    ///   the number of qubits in the register [`get_num_qubits()`]
    /// - If `Qureg` is a density matrix, this is equal to `2^(2N)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &Qureg::try_new_density(3, env).unwrap();
    ///
    /// assert_eq!(qureg.get_num_amps_total(), 64);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`get_num_qubits()`]: crate::Qureg::get_num_qubits()
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[must_use]
    pub fn get_num_amps_total(&self) -> i64 {
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
    /// let env = &QuestEnv::new();
    /// let qureg = &create_qureg::<2>(env);
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
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<2>(env);
    ///
    /// init_blank_state(qureg);
    ///
    /// assert!(get_prob_amp(qureg, 0).unwrap().abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 1).unwrap().abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 2).unwrap().abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 3).unwrap().abs() < EPSILON);
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
    /// - `qureg`: a [`Qureg`][api-qureg] of which to clear all amplitudes
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<2>(env);
    ///
    /// init_zero_state(qureg);
    ///
    /// assert!((get_prob_amp(qureg, 0).unwrap() - 1.).abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 1).unwrap().abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 2).unwrap().abs() < EPSILON);
    /// assert!(get_prob_amp(qureg, 3).unwrap().abs() < EPSILON);
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
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<2>(env);
    ///
    /// init_plus_state(qureg);
    ///
    /// assert!((get_prob_amp(qureg, 0).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((get_prob_amp(qureg, 1).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((get_prob_amp(qureg, 2).unwrap() - 0.25).abs() < EPSILON);
    /// assert!((get_prob_amp(qureg, 3).unwrap() - 0.25).abs() < EPSILON);
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
    ///  - `qureg`: the register to modify
    ///  - `state_ind` the index of the basis state to modify `qureg` into
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `state_ind` is outside [0, qureg.[`num_qubits_represented()`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<3>(env);
    ///
    /// init_classical_state(qureg, 8);
    /// let prob = get_prob_amp(qureg, 0).unwrap();
    ///
    /// assert!((prob.abs() - 1.) < EPSILON);
    /// ```
    ///
    ///
    /// See [QuEST API] for more information.
    ///
    /// [api-get-prob-amp]: crate::get_prob_amp()
    /// [`num_qubits_represented()`]: crate::Qureg::num_qubits_represented()
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
    /// - `qureg`: the register to modify
    /// - `pure`: a state-vector containing the pure state into which to
    ///   initialize `qureg`
    ///
    /// # Errors
    ///
    /// - [`InvalidQuESTInputError`],
    ///   - if `qureg` and `pure` have mismatching dimensions
    ///   - if `pure` is a density matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut Qureg::try_new_density(3, env).unwrap();
    /// let pure_state = &mut create_qureg::<3>(env);
    ///
    /// init_zero_state(pure_state);
    /// init_pure_state(qureg, pure_state).unwrap();
    ///
    /// assert!((calc_purity(qureg).unwrap() - 1.0).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`InvalidQuESTInputError`]: crate::QuestError::InvalidQuESTInputError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_pure_state(
        &mut self,
        pure_: &Qureg<'_, N>,
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
    /// [`qureg.get_num_amps_total()`]. There is no automatic checking that the
    /// passed arrays are L2 normalized, so this can be used to prepare `qureg`
    /// in a non-physical state.
    ///
    /// In distributed mode, this would require the complete state to fit in
    /// every node. To manually prepare a state for which all amplitudes cannot
    /// fit into a single node, use [`set_amps()`]
    ///
    /// # Parameters
    ///
    /// - `qureg`: the register to overwrite
    /// - `reals`: array of the real components of the new amplitudes
    /// - `imags`: array of the imaginary components of the new amplitudes
    ///
    /// # Errors
    ///
    /// - [`ArrayLengthError`],
    ///   - if either `reals` or `imags` have fewer than
    ///     [`qureg.get_num_amps_total()`] elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<2>(env);
    ///
    /// init_state_from_amps(qureg, &[1., 0., 0., 0.], &[0., 0., 0., 0.]);
    /// let prob = get_prob_amp(qureg, 0).unwrap();
    ///
    /// assert!((prob - 1.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`qureg.get_num_amps_total()`]: crate::Qureg::get_num_amps_total()
    /// [`set_amps()`]: crate::set_amps()
    /// [`ArrayLengthError`]: crate::QuestError::ArrayLengthError
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn init_state_from_amps(
        &mut self,
        reals: &[Qreal],
        imags: &[Qreal],
    ) -> Result<(), QuestError> {
        let num_amps_total = self.get_num_amps_total() as usize;
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
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<3>(env);
    ///
    /// let re = &mut [1., 2., 3., 4.];
    /// let im = &mut [1., 2., 3., 4.];
    /// set_amps(qureg, 0, re, im);
    ///
    /// // modify re and im to the next set of elements
    /// for i in 0..4 {
    ///     re[i] += 4.;
    ///     im[i] += 4.;
    /// }
    /// set_amps(qureg, 4, re, im);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `qureg`: the state-vector to modify
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
    ///   - if `start_ind` is outside [0, [`qureg.get_num_amps_total()`]]
    ///   - if `reals.len()` is outside [0, `qureg.get_num_amps_total()`]
    ///   - if `reals.len()` + `start_ind` >= `qureg.get_num_amps_total()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<2>(env);
    ///
    /// let re = &[1., 2., 3.];
    /// let im = &[4., 5., 6.];
    /// let start_ind = 1;
    /// set_amps(qureg, start_ind, re, im);
    ///
    /// let amp = get_real_amp(qureg, 3).unwrap();
    /// assert!((amp - 3.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`qureg.get_num_amps_total()`]: crate::Qureg::get_num_amps_total()
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
    /// - `qureg`: the state-vector to modify
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
    ///   - if `start_row` is outside [0, 1 <<
    ///     [`qureg.num_qubits_represented()`]]
    ///   - if `start_col` is outside [0, 1 <<
    ///     [`qureg.num_qubits_represented()`]]
    ///   - if `reals.len()` is outside [0, `qureg.get_num_amps_total()`]
    ///   - if `reals.len()` is larger than the remaining number of amplitudes
    ///     from (`start_row`, `start_col`), column-wise
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut Qureg::try_new_density(2, env).unwrap();
    ///
    /// let re = &[1., 2., 3.];
    /// let im = &[4., 5., 6.];
    /// let start_row = 1;
    /// let start_col = 1;
    /// set_density_amps(qureg, start_row, start_col, re, im);
    ///
    /// let amp = get_density_amp(qureg, 2, 1).unwrap();
    ///
    /// assert!((amp.re - 2.).abs() < EPSILON);
    /// assert!((amp.im - 5.).abs() < EPSILON);
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [`set_amps()`]: crate::set_amps()
    /// [`num_qubits_represented()`]: crate::Qureg::num_qubits_represented()
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
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<3>(env);
    ///
    /// let target_qubit = 1;
    /// let angle = 0.5;
    ///
    /// phase_shift(qureg, target_qubit, angle).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
    /// [QuEST API]: https://quest-kit.github.io/QuEST/modules.html
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn phase_shift(
        &mut self,
        target_quibit: i32,
        angle: Qreal,
    ) -> Result<(), QuestError> {
        catch_quest_exception(|| unsafe {
            ffi::phaseShift(self.reg, target_quibit, angle);
        })
    }

    /// Introduce a phase factor on state of qubits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<3>(env);
    ///
    /// let id_qubit1 = 0;
    /// let id_qubit2 = 2;
    /// let angle = 0.5;
    /// controlled_phase_shift(qureg, id_qubit1, id_qubit2, angle).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
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
    /// # Examples
    ///
    /// ```rust
    /// # use quest_bind::*;
    /// let env = &QuestEnv::new();
    /// let qureg = &mut create_qureg::<4>(env);
    ///
    /// let control_qubits = &[0, 1, 3];
    /// let angle = 0.5;
    /// multi_controlled_phase_shift(qureg, control_qubits, angle).unwrap();
    /// ```
    ///
    /// See [QuEST API] for more information.
    ///
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
} // Qureg

impl<'a, const N: u16> Drop for Qureg<'a, N> {
    fn drop(&mut self) {
        catch_quest_exception(|| {
            unsafe { ffi::destroyQureg(self.reg, self.env.0) };
        })
        .expect("dropping Qureg should always succeed");
    }
}

pub fn create_qureg<const N: u16>(env: &QuestEnv) -> Qureg<'_, N> {
    Qureg::try_new(env).expect("cannot allocate new Qureg")
}

pub fn create_density_qureg<const N: u16>(env: &QuestEnv) -> Qureg<'_, N> {
    Qureg::try_new_density(env).expect("cannot allocate new Qureg")
}
