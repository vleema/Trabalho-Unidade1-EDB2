#include <iostream>
#include <vector>

void troca(std::vector<int> &A, int i, int j){
    int aux;
    aux = A[i];
    A[i] = A[j];
    A[j] = aux;
}

int particao(std::vector<int> &A, int p, int r){
    int pivo, i, j;
    pivo = A[(p+r)/2]; 
    i = p-1;
    j = r+1;

    while(i < j){
        do{
            j--;
        }while(A[j] > pivo);
        do{
            i++;
        }while(A[i] < pivo);
        if(i < j){
            troca(A, i, j);
        }
    }
    return j;
}

void quickSort(std::vector<int> &A, int p, int r){
    int q;
    if(p < r){
        q = particao(A, p, r);
        quickSort(A, p, q);
        quickSort(A, q + 1, r);
    }
}

int main(){
    std::vector<int> A = {5, 8, 3, 1, 6, 2, 4, 9, 7, 5};

    quickSort(A, 0, 9);

    for(int ii = 0; ii < 9; ii++){
        std::cout << A[ii] << std::endl;
    }
}