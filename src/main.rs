use miden_processor::ExecutionOptions;
use miden_vm::{
    execute, execute_iter, prove, verify, Assembler, DefaultHost, ProvingOptions, StackInputs,
};

fn main() {
    let assember = Assembler::default();
    let program = assember
        .compile(
            r#"
            begin push.1 push.7 add end
        "#,
        )
        .unwrap();

    let stack_input = StackInputs::default();

    let mut host = DefaultHost::default();

    let exec_options = ExecutionOptions::default();

    let prove_options = ProvingOptions::default();

    let _trace = execute(&program, stack_input.clone(), &mut host, exec_options).unwrap();

    let (program_outputs, proofs) =
        prove(&program, stack_input.clone(), &mut host, prove_options).unwrap();

    println!("program_outputs: {:?}", &program_outputs);
    println!("proofs: {:?}", &proofs.proof);

    let security_level = verify(
        program.clone().into(),
        stack_input.clone(),
        program_outputs,
        proofs,
    )
    .unwrap();

    println!("security level: {}", security_level);

    for vm_state in execute_iter(&program, stack_input, host) {
        match vm_state {
            Ok(vm_state) => println!("{:?}", vm_state),
            Err(_) => println!("something went terribly wrong!"),
        }
    }
}
