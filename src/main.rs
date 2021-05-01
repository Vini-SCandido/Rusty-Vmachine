use vmachine2::cpu::Stackvm;
fn main() {
    let mut vm = Stackvm::new(vec![0; 1000000]);
    let program = vec![3, 4, 0x40000001, 5, 0x40000002, 3, 0x40000003, 2, 0x40000004, 0x40000000];
    vm.load_program(program);
    vm.run();
}