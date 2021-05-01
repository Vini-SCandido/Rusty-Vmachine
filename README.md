# Rusty-Vmachine

Rusty Vmachine é um pequeno projeto que me ajudou a entender um pouco mais sobre o funcionamento dos computadores, linguagem assembly, emuladores e máquinas virtuais. É um programa que simula um pequeno computador, com seus componentes, como memória e a stack, descritos em listas.

## Uso

Como é escrito em rust, é necessário ter o compilador.
Veja [Instalando Rust](https://www.rust-lang.org/pt-BR/tools/install)

Você pode usar git clone para transferir esse repositório para sua máquina, e após a instalação, basta entrar na pasta e digitar:
```
cargo run --release
```

## Passando um programa
```rust
use vmachine2::cpu::Stackvm; // importa o struct Stackvm, que representa a máquina virtual
fn main() {
    let mut vm = Stackvm::new(vec![0; 1000000]); //Aqui é "vm" é declarado como instância da máquina virtual.
    // Esse é o programa contendo as instruções.
    let program = vec![3, 4, 0x40000001, 5, 0x40000002, 3, 0x40000003, 2, 0x40000004, 0x40000000];
    vm.load_program(program); // carrega o prgrama para dentro da memória
    vm.run(); // roda o programa
}
```
Nesse exemplo, números como 3 e 4 são armazenados no topo da stack, e os demais números são comandos matemáticos que atuam nos dois elementos do topo da stack. Note, que eles estão na base hexadecimal, mas não faz diferença representar tudo em hexadecimal. É só um meio de visualizar melhor os comandos entre a informação nos quais atuam.

```rust
0x40000001 // operação de adição
0x40000002 // operação de substração
0x40000003 // operação de multiplicação
0x40000004 // operação de divisão (divisão inteira)

0x40000000 // encerra o programa
```
## Exemplo de execução

```
Tos:     3  // tos significa top of stack, que é o elemtento mais alto da pilha. É sempre mostrado
Tos:     4
Add:     3     4
Tos:     7
Tos:     5
Sub:     7     5
Tos:     2
Tos:     3
Mul:     2     3
Tos:     6
Tos:     2
Div:     6     2
Tos:     3
Halt:  ---- ----
Tos:     3
```
## Contribuindo

Como estou aprendendo a linguaguem, você pode deixar algum comentário sobre como posso melhorar meu código

## Reconhecimentos

Só consegui chegar até aqui com a ajuda [desse tutorial](https://youtu.be/BNXP0w4Ppto). É escrito em c++, então meu código é apenas uma tradução do original.
O repositório do tutorial pode ser encontrado aqui: https://github.com/pbohun/stack-vm-tutorials

