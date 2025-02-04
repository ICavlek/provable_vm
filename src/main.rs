use provable_vm::provable_vm::program_loader::load_program;

fn main() {
    let program = load_program("./examples/program.prov").expect("Failed to load program");
    println!("{:#?}", program);
}
