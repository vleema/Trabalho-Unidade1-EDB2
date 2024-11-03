#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <vector>

#include "../generate_report.cpp"

using namespace std;

bool idadeRep2(vector<int> &idade) {
  sort(idade.begin(), idade.end());
  return idade[0] == idade[1];
}

bool idadeRep(vector<int> &idade) {
  int tam = idade.size();
  int indexMenor = 0;
  for (int i = 1; i < tam; i++) {
    if (idade[i] < idade[indexMenor]) {
      indexMenor = i;
    }
  }
  for (int i = 0; i < tam; i++) {
    if (idade[i] == idade[indexMenor] && i != indexMenor) {
      return true;
    }
  }
  return false;
}

int main(int argc, char *argv[]) {
  ofstream file("output.txt");

  if (!file) {
    cerr << "Error on output file." << endl;
    return 1;
  }

  generate_report(file, {100, 1000, 10000}, "idadeRep", idadeRep);
  generate_report(file, {100, 1000, 10000}, "idadeRep2", idadeRep2);

  file.close();

  return 0;
}
