use miden_vm::{execute_iter, verify, DefaultHost, ProvingOptions, StackInputs};
use miden_prover::prove;
use miden_assembly::Assembler;

fn main() {
    let assember = Assembler::default();
    let program = assember
        .assemble_program(
            r#"
            begin push.2 push.7 add end
        "#,
        )
        .unwrap();

    let stack_input = StackInputs::default();

    let mut host = DefaultHost::default();

    //try use hash_FN rpo256 but slowly and Blake3_192 by default
    let prove_options = ProvingOptions::with_96_bit_security(true);

    /*
    use miden_processor::ExecutionOptions;
    use minden_vm::{execute,}
    execution function included in the prove function
    let exec_options = ExecutionOptions::default();
    let _trace = execute(&program, stack_input.clone(), &mut host, exec_options).unwrap();
    */

    let (program_outputs, proofs) =
        prove(&program, stack_input.clone(), &mut host, prove_options).unwrap();

    println!("program_outputs: {:?}", &program_outputs);
    // println!("proofs: {:?}", &proofs.proof.fri_proof);

    let security_level =
        verify(program.clone().into(), stack_input.clone(), program_outputs, proofs).unwrap();

    println!("security level: {}", security_level);

    /*
    VmState { clk: 0, ctx: ContextId(0), op: None, asmop: None, fmp: 1073741824, stack: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 1, ctx: ContextId(0), op: Some(Span), asmop: None, fmp: 1073741824, stack: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 2, ctx: ContextId(0), op: Some(Push(2)), asmop: None, fmp: 1073741824, stack: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 3, ctx: ContextId(0), op: Some(Push(7)), asmop: None, fmp: 1073741824, stack: [7, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 4, ctx: ContextId(0), op: Some(Add), asmop: None, fmp: 1073741824, stack: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 5, ctx: ContextId(0), op: Some(Noop), asmop: None, fmp: 1073741824, stack: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    VmState { clk: 6, ctx: ContextId(0), op: Some(End), asmop: None, fmp: 1073741824, stack: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [] }
    */

    for vm_state in execute_iter(&program, stack_input, host) {
        match vm_state {
            Ok(vm_state) => println!("{:?}", vm_state),
            Err(_) => println!("something went terribly wrong!"),
        }
    }
}
