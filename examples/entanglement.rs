//! This is a basic example showing how to initialize `QuEST` environment
//! and perform a operations on a quantum register consisting of 2 qubits.
//!
//! We entangle the qubits by preparing a Bell state `|00> + |11>`.
//! Next, we measure both qubits in the computational basis.  Because qubits are
//! entangled, after the measurement they are both in the same, equally probable
//! state `0` or `1`.
use quest_bind::{
    QuestEnv,
    QuestError,
    Qureg,
};

fn main() -> Result<(), QuestError> {
    // Initialize QuEST environment and report to screen
    let env = &QuestEnv::new();
    env.report_quest_env();

    // Create a 2-qubit register and report its parameters
    let mut qureg = Qureg::try_new(2, env).expect("cannot allocate new Qureg");
    qureg.report_qureg_params();
    // Initialize |00> state and print out the state to screen
    qureg.init_zero_state();
    qureg.report_state_to_screen(0);

    // Prepare a Bell state `|00> + |11>`: apply Hadamard gate
    // on qubit 0, then NOT on qubit 1, controlled by qubit 0.
    println!("---\nPrepare Bell state: |00> + |11>");
    qureg.hadamard(0).and(qureg.controlled_not(0, 1))?;

    // Measure both qubits
    let outcome0 = qureg.measure(0)?;
    let outcome1 = qureg.measure(1)?;
    println!("Qubit \"0\" measured in state: |{outcome0}>");
    println!("Qubit \"1\" measured in state: |{outcome1}>");

    // Because the state was entangled, the outcomes
    // should always be the same
    if outcome0 == outcome1 {
        println!("They match!");
        Ok(())
    } else {
        panic!("qubits in Bell state should be perfectly correlated");
    }

    // At this point both `qureg` and `env` are dropped and
    // the allocated memory is freed.
}
