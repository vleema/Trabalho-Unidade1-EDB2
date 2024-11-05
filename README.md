# Sobre o repositório

## Sumário

<!--toc:start-->

- [$\LaTeX$](#latex)
  - [Main](#main)
  - [Escrevendo](#escrevendo)
    - [Codeblock](#codeblock)
    - [Pseudocódigo](#pseudocódigo)
    - [Tabela](#tabela)
  - [Compilação](#compilação)
- [Arquivos de código](#arquivos-de-código)
  - [Rust](#rust)
  - [C++](#c)
  <!--toc:end-->

## $\LaTeX$

A partir de uma decisão democrática (minha), resolvemos fazer esse trabalho em $\LaTeX$ com todas as suas dependências nesse repositório, então, nas próximas linhas vou passar algumas utilidades para a gente trabalhar direitinho e tirar um 10 nesse troço.

### Main

O nosso código em Latex está organizado da seguinte forma:

```plain-text
.
├── chapters
│   ├── 01_introduction.tex
│   ├── 02_literature.tex
│   ├── 03_methodology.tex
│   ├── 04_results.tex
│   ├── 05_discussion.tex
│   ├── 06_conclusions.tex
│   └── 07_reflection.tex
├── figures
│   ├── chart.pdf
│   └── ufrnlogo.png
├── CS_report.sty
├── main.tex
└── references.bib
```

No `main.tex`, está o esqueleto do projeto, como o import do pacote `CS_report.sty` que contém definições, configurações relacionadas ao projeto, página inicial, integrantes do grupo, etc. Além disso, contém a chamada dos arquivos de outras seções do documento, com:

```latex
    \input{chapters/01_introduction}
    \input{chapters/02_literature} % https://guides.library.bloomu.edu/litreview
    \input{chapters/03_methodology}
    \input{chapters/04_results}
    \input{chapters/05_discussion}
    \input{chapters/06_conclusions}
    \input{chapters/07_reflection}
```

E aí quando for escrever uma seção, basta adicionar aqui ou modificar um existente.

### Escrevendo

Latex é uma linguagem relativamente simples e possui muita documentação sobre, sempre uso o <https://www.overleaf.com/learn/latex/Tutorials> para consultar sintaxe e o que desejo fazer, o chatGPT ajuda também, mas ele é um pouquinho besta pra $\LaTeX$.

Nesse trabalho vai ser necessário escrever **código**, **pseudocódigo** e listar os resultados de performance obtidos numa tabela por exemplo, para isso, podemos seguir esses exemplinhos.

> [!CAUTION]
> As vezes a inserção figuras, tabelas, blocos de códigos pode ocorrer em cantos que você não necessariamente espera. Para evitar ou corrigir isso, use a função `\FloatBarrier` após a inserção de algum elemento gráfico.

#### Codeblock

##### Documento

![codeblock](./assets/codigo.png)

##### Código

```latex
\begin{lstlisting}[language=Python, caption={Code snippet in \LaTeX ~and  this is a Python code example}, label=list:python_code_ex]
import numpy as np

x  = [0, 1, 2, 3, 4, 5] # assign values to an array
evenSum = evenSummation(x) # call a function

def evenSummation(x):
    evenSum = 0
    n = len(x)
    for i in range(n):
        if np.mod(x[i],2) == 0: # check if a number is even?
            evenSum = evenSum + x[i]
    return evenSum
\end{lstlisting}
```

#### Pseudocódigo

##### Documento

![pseudocodigo](./assets/pseudocodigo.png)

##### Código

```latex
\begin{algorithm}
    \caption{Example caption: sum of all even numbers}
    \label{algo:algo_example}
    \begin{algorithmic}[1]
        \Require{$ \mathbf{x}  = x_1, x_2, \ldots, x_N$}
        \Ensure{$EvenSum$ (Sum of even numbers in $ \mathbf{x} $)}
        \Statex
        \Function{EvenSummation}{$\mathbf{x}$}
        \State {$EvenSum$ $\gets$ {$0$}}
        \State {$N$ $\gets$ {$length(\mathbf{x})$}}
        \For{$i \gets 1$ to $N$}
        \If{$ x_i\mod 2 == 0$}  \Comment Check whether a number is even.
        \State {$EvenSum$ $\gets$ {$EvenSum + x_i$}}
        \EndIf
        \EndFor
        \State \Return {$EvenSum$}
        \EndFunction
    \end{algorithmic}
\end{algorithm}
```

#### Tabela

##### Documento

![tabela](./assets/tabela.png)

##### Código

```latex
\begin{table}[h!]
    \centering
    \caption{Example of a table in \LaTeX}
    \label{tab:_ex_tab}
    \begin{tabular}{llr}
        \toprule
        \multicolumn{2}{c}{Bike} \\
        \cmidrule(r){1-2}
        Type    &  Color & Price (\pounds) \\
        \midrule
        Electric    & black   & 700   \\
        Hybrid      & blue    & 500   \\
        Road        & blue    & 300   \\
        Mountain    & red     & 300   \\
        Folding     & black   & 500   \\
        \bottomrule
    \end{tabular}
\end{table}
```

#### Diagramas

##### Documento

![diagram-example](./assets/diagrama.png)

##### Código

Use o [quiver](https://q.uiver.app/), lá tem a opção de pegar o código fonte.

### Compilando

Particularmente eu uso o plugin <https://github.com/lervag/vimtex> no neovim com a opção:

```lua
vim.g.vimtex_compiler_latexmk = {
  out_dir = "output",
}
```

Para gerar os arquivos de compilação dentro da pasta relativa `output`. E aí enquanto escrevo e salvo o neovim já me mostra o pdf em tempo real enquanto edito o texto.

Algumas alternativas para codar e compilar latex seriam:

- **Usar o [Latex Workshop](https://github.com/James-Yu/LaTeX-Workshop) no vscode**.

  > Tutorial prático de como configurar: <https://mathjiajia.github.io/vscode-and-latex/>

- **Usar o overleaf e depois copiar de volta pro repositório**.

  > Basta ir em New project > Blank Project, uploada tudo que tem na pasta latex pro overleaf e edita por lá.

- **Compilar manualmente e verificar o pdf**.
  > Para isso, deixei um `Makefile` dentro da pasta `latex`. Basta executar `make` e o resultado vai estar em `output`

Se for usar o Latex Workshop no vscode, lembre de especificar o diretório que terá o resultado da compilação (`output`), se não os arquivos não vão ser barrados pelo `.gitignore`.

## Arquivos de código

Os arquivos de código que contem as implementações requeridas estão na pasta `src/`, em Rust estão sendo implementados os algoritmos de ordenação e em C++ foram implementados os `idadeRep` e provavelmente serão implementados os códigos de busca.

Antes que me julguem por colocar Rust no meio, o código no estado atual (2024-10-23 22:13), já existia desde muito tempo, então só reaproveitei e implementei a parte que analisa a performance.

### Rust

#### Rodando

Pra rodar basta executar dentro da pasta `src/rust`:

```terminal
cargo run
```

E aí ele vai exibir criar o arquivo `src/rust/src/out/entries.txt` e `src/rust/src/out/output.txt`, com as lisas que ele usou e o resultado de performance respectivamente.

### C++

Estando em `src/cpp`, compile e execute `<algoritmo>/main.cpp`. A saída estará na mesma pasta que foi executado.

# To-do

## 1. Análise Teórica

- [ ] 1.0 Deve ser feita a descrição das funções/procedimentos utilizados e uma explicação do funcionamento geral para o algoritmo (pseudocódigo), buscando justificar o resultado esperado em todos os casos possíveis (explicar porque o seu algoritmo funciona).
- [ ] 1.1 Estabeleça pseudocódigos para o algoritmo de ordenação BubbleSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos.
- [~] 1.2 Estabeleça pseudocódigos para o algoritmo de ordenação MergeSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos.
  > Falta o pseudo it, analise de complexidade das duas versões
- [ ] 1.3 Estabeleça pseudocódigos para o algoritmo de ordenação QuickSort em versões iterativa e recursiva e analise a complexidade em relação as notações Big O, ômega e Theta. Para a versão recursiva, utilize os 4 métodos vistos.

## 2. Fase de análise de algoritmo

- [x] 2.0 Descreva o ambiente computacional utilizado (Software e Hardware).**(Bibs)**
- [x] 2.1 Função iterativa
  - [x] 2.1.A Implemente os códigos idadeRep e idadeRep2, vistos em sala de aula. **(Oaks)**
  - [x] 2.1.B Crie instâncias aleatórias com idades (inteiros) variando entre 0 e 100, com tamanhos n=100, n=1.000, n=10.000. **(Oaks)**
  - [x] 2.1.C Compute os tempos de processamento e compare com os resultados da análise assintótica vista em sala. **(Oaks)**
- [x] 2.2 Função recursiva
  - [x] 2.2.A Implemente os códigos buscaBinaria e bBinRec, vistos em sala de aula. **(Bibs)**
  - [x] 2.2.B Crie instâncias aleatórias com idades (inteiros) variando entre 0 e 100, com tamanhos n=100, n=1.000, n=10.000 **(Bibs)**
  - [x] 2.2.C Compute os tempos de processamento e compare com os resultados da análise assintótica vista em sala.**(Bibs)**
- [x] 2.3 Implemente os algoritmos de ordenação baseados nos pseudocódigos que você sugeriu nos itens 1.A, 1.B e 1.C, em suas versões iterativa e recursiva. **(Lims)**
- [x] 2.4 Crie listas aleatórias, com inteiros variando entre 0 e 1000, de tamanho 1.000, 10.000 e 100.000(salve as listas em arquivo txt). **(Lims)**
- [x] 2.5 Execute os algoritmos de ordenação (BubbleSort, MergeSort e QuickSort, iterativo e recursivo) nas listas, salve o resultado da lista ordenada em um arquivo txt, e compute o tempo de processamento para cada caso. Faça o tabelamento dos resultados, e realize uma análise detalhada. Esta análise deve constar no arquivo PDF enviado. **(Lims)**

## 3. Formalizar no Latex

> 4 pra cada?

- [x] 1.0 marina @
- [x] 1.1 marina @
- [x] 1.2 lime, oaks
- [x] 1.3 marina @
- [x] 2.0 bianca
- [x] 2.1
  - [x] 2.1.A oaks
  - [x] 2.1.B oaks
  - [x] 2.1.c oaks
- [x] 2.2
  - [x] 2.2.A bianca
  - [x] 2.2.B bianca
  - [x] 2.2.C bianca
- [x] 2.3 lime
- [x] 2.4 lime
- [x] 2.5 lime

---

&copy; IMD/UFRN 2024
