# Resoluções

### Slide_004 Notação Ω, Θ e Big O

#### 1

> Faça a tabela  assintótica para os códigos `idadeRep` e `idadeRep2`.

Note que $\texttt{idadeRep}$ pode ser matemáticamente como: $\texttt{idadeRep}(n) = (c_1 + c_2) \cdot n$, onde $c_1$ e $c_2$ são as operações que ocorrem dentro do `for` e $n$ o tamanho da lista. Portanto, a tabela assintótica pode ser modelada como:

- $\texttt{idadeRep}$

|                 | Big O (Pior Caso)            | Ω (Melhor caso)               | Θ (Média)               |
| --------------- | ---------------              | ---------------               | ---------------         |
| $n!$            | N                            | N                             | N                       |
| $2^n$           | N                            | N                             | N                       |
| $n^2$           | N                            | N                             | N                       |
| $n \log n$      | N                            | N                             | N                       |
| $n$             | S                            | N                             | S                       |
| $\log n$        | N                            | N                             | N                       |
| 1               | N                            | S                             | N                       |

- $\texttt{idadeRep2}$

A complexidade dessa função será dada pela chamada da função  `std::sort` dentro de sua implementação somado a constante de comparação, que de acordo  com a documentação (<https://en.cppreference.com/w/cpp/algorithm/sort>) é:

|                 | Big O (Pior Caso)            | Ω (Melhor caso)               | Θ (Média)               |
| --------------- | ---------------              | ---------------               | ---------------         |
| $n!$            | N                            | N                             | N                       |
| $2^n$           | N                            | N                             | N                       |
| $n^2$           | N                            | N                             | N                       |
| $n \log n$      | S                            | S                             | S                       |
| $n$             | N                            | N                             | N                       |
| $\log n$        | N                            | N                             | N                       |
| 1               | N                            | N                             | N                       |

#### 2

> Faça a tabela  assintótica para os códigos `BuscaLista2` e `BuscaLista3`
