# vigenere-cipher
## Pre-requisitos
- Linux (pelo menos wsl)
- Rust
## How to build
Para executar basta ter o [Rust](https://www.rust-lang.org/tools/install) mais recente instalado e rodar o seguinte comando na pasta do projeto:
```
cargo run --release
```

## How to run
### Solver
Usado para quebrar uma cifra com chave desconhecida
```
cargo run --release -- solver <path_para_arquivo_log> <path_para_arquivo_da_cifra> <path_da_saida> -k <tamanho_maximo_key>
```
> A senha terá o output no terminal
> arquivo log é o arquivo que contem a probabilidades em log normalizadas
### Decipher
Usado para descifrar uma cifra com chave conhecida
```
cargo run --release -- decipher <path_para_cifra> <path_para_arquivo_saida>
```
### Cipher
Usado para cifrar um texto com uma chave especifica
```
cargo run --release -- cipher <path_para_texto> <path_para_arquivo_saida>
```