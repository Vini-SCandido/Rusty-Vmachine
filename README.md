# Rusty Vmachine

Rusty Vmachine é um pequeno projeto que me ajudou a entender um pouco mais sobre o funcionamento dos computadores, linguagem assembly, emuladores e máquinas virtuais. É um programa que simula um pequeno computador, com seus componentes, como memória e os registradores, descritos em listas.

## Por que Rust?

Rust é uma linguagem que tem crescido muito nos últimos tempos, sendo classificada como uma das linguagens mais favoritas pelo Stack Overflow. Suas características centrais envolvem o uso adequado de memória, que previne muitos bugs que acontecem em linguagens como C/C++, e rapidez.

Rust é apontada como uma linguagem de baixo nível, por isso é ideal para esse projeto, além do fato que de colabora para a compreensão de vários tópicos dentro da área de sistemas.

Você pode começar a estudar a linguagem aqui: [Aprenda Rust](https://www.rust-lang.org/pt-BR/learn)

## Uso

[Instale o rustup](https://www.rust-lang.org/pt-BR/tools/install). Clone o repositório e passe o seguinte comando:
```
cargo run
```

## Passando um programa
```rust
// importa o struct Stackvm, que representa a máquina virtual
use vmachine2::cpu::Stackvm; 

fn main() {
    //Aqui é "vm" é declarado como instância da máquina virtual.
    let mut vm = Stackvm::new(vec![0; 1000000]); // o parametro em "new" corresponde à um array de 1.000.000 elementos
                                                 // que representa a memória. 
    // Esse é o programa contendo as instruções.
    let program = vec![3, 4, 0x40000001, 5, 0x40000002, 3, 0x40000003, 2, 0x40000004, 0x40000000];
    // carrega o programa para dentro da memória
    vm.load_program(program); 
    // roda o programa
    vm.run(); 
}
```
Nesse exemplo, números como 3 e 4 são armazenados no topo da stack, e os demais números são comandos matemáticos que atuam nos dois elementos do topo dela. Note, que eles estão na base hexadecimal, mas não faz diferença representar tudo em hexadecimal. É só um meio de visualizar melhor os comandos entre a informação nos quais atuam.

Você pode encontrar toda a lógica por trás da leitura e interpretação dos programas executados em [/src/cpu.rs](src/cpu.rs)

```rust
0x40000001 // operação de adição
0x40000002 // operação de substração
0x40000003 // operação de multiplicação
0x40000004 // operação de divisão (divisão inteira)

0x40000000 // encerra o programa
```
A máquina virtual consegue distinguir os comandos dentre informações por causa do 31º bit
```rust
0x40000001 == 01000000000000000000000000000001 // <- o primeiro bit diz o tipo do comando. Nesse caso, adição.
               ^
              quando esse bit é 1, temos um comando
              
0x40000001 == 1073741825 // outra forma de representar o comando, nesse caso, em base decimal. Como dito anteriormente,
                        // é mais facil identificar o comando quando ele esta escrito em base hexadecimal.
```

## Exemplo de execução

Programa:
```rust
let program = vec![3, 4, 0x40000001, 5, 0x40000002, 3, 0x40000003, 2, 0x40000004, 0x40000000];
```
```
Tos:    3         // tos significa top of stack, que é o elemento mais alto da pilha. É sempre mostrado
Tos:    4
Add:    3    4    // 3+4
Tos:    7         // O Resultado sobe para o topo da pilha
Tos:    5
Sub:    7    5    // 7-5
Tos:    2         // Resultado da subtração
Tos:    3
Mul:    2    3    // 3*2
Tos:    6         // Resultado da multiplicação
Tos:    2
Div:    6    2    // 6/2
Tos:    3         // Resultado da divisão
Hlt: ---- ----    // fim
Tos:    3         // Novamente, o topo da pilha (stack).
```
## Contribuindo

Como estou aprendendo a linguaguem, você pode deixar algum comentário sobre como posso melhorar meu código

## Reconhecimentos

Só consegui chegar até aqui com a ajuda [desse tutorial](https://youtu.be/BNXP0w4Ppto). É escrito em C++, então meu código é apenas uma reformulação dos conceitos apresentados, traduzidos para a linguagem rust.
O repositório do tutorial pode ser encontrado aqui: https://github.com/pbohun/stack-vm-tutorials

