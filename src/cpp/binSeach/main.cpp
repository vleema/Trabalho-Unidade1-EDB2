#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
#include "../generate_report.cpp"

int buscaBinaria(vector<int> &lista, int tamanho, int x) {
    sort(lista.begin(), lista.end());
    int esq = 0; 
    int dir = tamanho - 1;

    while (esq < dir) {
        int m = (esq + dir) / 2;
        if (x > lista[m]) {
            esq = m + 1;
        } else {
            dir = m;
        }
    }


    if (lista[esq] == x) {
        return esq;
    }

    return -1;
}

int bBinRec(vector<int> &lista,int esq, int dir, int x ){
    sort(lista.begin(), lista.end());
    if (esq>dir) {
        return -1;
    }

    int m = (esq + dir) / 2;

    if (lista[m] == x) {
        return m;
    }

    if (lista[m] > x){
        return bBinRec(lista, esq, m - 1, x);
    }else{
        return bBinRec(lista, m + 1, dir, x);
    }
}

int main(int argc, char *argv[]){
    ofstream file("output.txt");

  if (!file) {
    cerr << "Error on output file." << endl;
    return 1;
  }
  
  generate_report(file, {100, 1000, 10000}, "buscaBinaria", buscaBinaria);
  generate_report(file, {100, 1000, 10000}, "bBinRec", bBinRec);

  file.close();

  return 0;
}
