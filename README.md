# Sobre o repositório

## $\LaTeX$

Para compilar o documento, use o Makefile na pasta latex.

- Compila o arquivo `main.pdf` em `latex/output`.

```terminal
make
```

- Limpa os arquivos de compilação

```terminal
 make clean
```

## Implementação dos algoritmos

Os arquivos de código que contem as implementações requeridas estão na pasta `src/`, em Rust estão sendo implementados os algoritmos de ordenação e em C++ foram implementados os `idadeRep`, `idadeRep2`, `bBinRec` e `binSearch`.

### Rust

#### Rodando

Pra rodar basta executar dentro da pasta `src/rust`:

```terminal
cargo run
```

E aí ele vai criar o arquivo `src/rust/src/out/entries.txt` e `src/rust/src/out/output.txt`, com as listas que ele usou e o resultado de performance respectivamente.

### C++

Estando em `src/cpp`, compile e execute `<algoritmo>/main.cpp`.

Por exemplo:

```terminal
clang++ -O3 binSearch/main.cpp
```

Um arquivo chamado `output.txt` vai ser criado na pasta atual.

# Lista de atividades dos colaboradores

## 1. Análise Teórica

- [x] 1.0 Deve ser feita a descrição das funções/procedimentos utilizados e uma explicação do funcionamento geral para o algoritmo (pseudocódigo), buscando justificar o resultado esperado em todos os casos possíveis (explicar porque o seu algoritmo funciona). (**Marina**)
- [x] 1.1 Estabeleça pseudocódigos para o algoritmo de ordenação BubbleSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos. (**Marina**)
- [x] 1.2 Estabeleça pseudocódigos para o algoritmo de ordenação MergeSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos. (**Vinicius e Gabriel**)
- [x] 1.3 Estabeleça pseudocódigos para o algoritmo de ordenação QuickSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos. (**Marina**)

## 2. Fase de análise de algoritmo

- [x] 2.0 Descreva o ambiente computacional utilizado (Software e Hardware).(**Vinicius**)
- [x] 2.1 Função iterativa
  - [x] 2.1.A Implemente os códigos idadeRep e idadeRep2, vistos em sala de aula. (**Gabriel**)
  - [x] 2.1.B Crie instâncias aleatórias com idades (inteiros) variando entre 0 e 100, com tamanhos n=100, n=1.000, n=10.000. (**Gabriel**)
  - [x] 2.1.C Compute os tempos de processamento e compare com os resultados da análise assintótica vista em sala. (**Gabriel**)
- [x] 2.2 Função recursiva
  - [x] 2.2.A Implemente os códigos buscaBinaria e bBinRec, vistos em sala de aula. (**Bianca**)
  - [x] 2.2.B Crie instâncias aleatórias com idades (inteiros) variando entre 0 e 100, com tamanhos n=100, n=1.000, n=10.000 (**Bianca**)
  - [x] 2.2.C Compute os tempos de processamento e compare com os resultados da análise assintótica vista em sala. (**Bianca**)
- [x] 2.3 Implemente os algoritmos de ordenação baseados nos pseudocódigos que você sugeriu nos itens 1.A, 1.B e 1.C, em suas versões iterativa e recursiva. (**Vinicius**)
- [x] 2.4 Crie listas aleatórias, com inteiros variando entre 0 e 1000, de tamanho 1.000, 10.000 e 100.000(salve as listas em arquivo txt). (**Vinicius**)
- [x] 2.5 Execute os algoritmos de ordenação (BubbleSort, MergeSort e QuickSort, iterativo e recursivo) nas listas, salve o resultado da lista ordenada em um arquivo txt, e compute o tempo de processamento para cada caso. Faça o tabelamento dos resultados, e realize uma análise detalhada. Esta análise deve constar no arquivo PDF enviado. (**Vinicius**)

Na verdade verdadeira, todo mundo se intrometeu em tudo! Mas por fins de prestar conta, essa é nossa lista.

---

&copy; IMD/UFRN 2024
