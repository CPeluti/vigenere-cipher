# vigenere-cipher
## How to build
Para executar basta ter o [Rust](https://www.rust-lang.org/tools/install) mais recente instalado e rodar o seguinte comando na pasta do projeto:
```
cargo run --release
```

## How to run
### Solver
```
cargo run --release -- solver <path_para_arquivo_log> <path_para_arquivo_da_cifra> <path_da_saida>
```
A senha terá o output no terminal
### Decipher
```
cargo run --release -- decipher <path_para_cifra> <path_para_arquivo_saida>
```
### Cipher
```
cargo run --release -- cipher <path_para_texto> <path_para_arquivo_saida>
```