
#include <algorithm>
#include <chrono>
#include <cstdlib>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

long double get_average(vector<chrono::duration<long double>> times) {
  chrono::duration<long double> sum = chrono::duration<long double>(0);

  for (const auto &elapsed : times) {
    sum += elapsed;
  }

  return sum.count() / times.size();
}

bool idadeRep2(vector<int> &idade) {
  sort(idade.begin(), idade.end());
  return idade[0] == idade[1];
}

bool idadeRep(vector<int> &idade) {
  int tam = idade.size();
  int menor = 200;
  for (int i = 0; i < tam; i++) {
    if (idade[i] < menor) {
      menor = idade[i];
    }
  }
  for (int i = 0; i < tam; i++) {
    if (idade[i] == menor) {
      return true;
    }
  }
  return false;
}

int main() {
  vector<int> vec;
  size_t how_many_lists = 5;
  ofstream output("output.txt"); // Define o arquivo de saída

  if (!output) {
    cerr << "Erro ao abrir o arquivo de saída." << endl;
    return 1;
  }

  for (const auto &algorithm : {"idadeRep", "idadeRep2"}) {
    output << algorithm << endl;

    for (const auto &n : {100, 1000, 10000}) {
      vector<chrono::duration<long double>> times;

      output << "   n=" << n << endl;

      for (size_t i = 1; i <= how_many_lists; i++) {
        ostringstream file_name;
        vector<int> list;

        // Carregando dados para a memória
        file_name << "../inputs/n=" << n << "/list-" << i << ".txt";

        ifstream file(file_name.str());
        if (!file) {
          cerr << "Erro ao abrir o arquivo de entrada: " << file_name.str()
               << endl;
          continue;
        }

        string line;
        while (getline(file, line)) {
          list.push_back(stoi(line));
        }

        // Análise
        if (algorithm == string("idadeRep")) {
          auto start = chrono::system_clock::now();

          idadeRep(list);

          auto end = chrono::system_clock::now();
          chrono::duration<long double> elapsed = end - start;
          times.push_back(elapsed);

          output << "      Tempo para idadeRep: " << fixed << setprecision(9)
                 << elapsed.count() << " segundos." << endl;
        } else if (algorithm == string("idadeRep2")) {
          auto start = chrono::system_clock::now();

          idadeRep2(list);

          auto end = chrono::system_clock::now();
          chrono::duration<long double> elapsed = end - start;
          times.push_back(elapsed);

          output << "      Tempo para idadeRep2: " << fixed << setprecision(9)
                 << elapsed.count() << " segundos." << endl;
        }
      }
      output << "   Average: " << get_average(times) << " seconds." << endl
             << endl;
    }
    output << endl;
  }

  return 0;
}
