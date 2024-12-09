# pcompiler

Compilador da **linguagem P**.

> A linguagem p é uma linguagem de programação didática criada pelo profesor 
Alexandre Bittencourt Pigozzo. O trabalho desenvolvido na matéria de compiladores 
da Universidade Federal de São João del-Rei no período 2024/2.

## *Build*

```bash
cargo fetch
cargo build --release

# O executável estará disponível como target/release/pcompiler.
```

## Execução

```
./target/release/pcompiler <source_file>
```

### Resultado parcial

Nesta versão, estão implementados apenas os analisadores sintático e léxico.
A execução gera então dois arquivos: *tokens*, que contém os tokens definidos 
durante a análise léxica, e *log*, que contém os erros encontrados ao longo da
execução de ambos os analisadores.
