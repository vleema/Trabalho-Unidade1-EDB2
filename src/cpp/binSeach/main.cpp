#include <iostream>
#include <vector>
using namespace std;
#include "../generate_report.cpp"

int buscaBinaria(vector<int> &lista, int tamanho, int x) {
    int esq = 0; 
    int dir = tamanho - 1;
    //std::cout << x << std::endl;

    while (esq <= dir) {
        int m = (esq + dir) / 2;
        //std::cout << lista[m] << std::endl;
        
        if (lista[m] == x) {
            //std::cout << " " << std::endl; 
            return m;
        } else if (x > lista[m]) {
            esq = m + 1;
        } else {
            dir = m - 1; 
        }
    }
    return -1; 
}

int bBinRec(vector<int> &lista,int esq, int dir, int x ){
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
    ofstream file("output1.txt");

  if (!file) {
    cerr << "Error on output file." << endl;
    return 1;
  }
  
  generate_report(file, {100, 1000, 10000}, "buscaBinaria", buscaBinaria);
  generate_report(file, {100, 1000, 10000}, "bBinRec", bBinRec);

  file.close();

  return 0;
}
