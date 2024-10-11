#include <algorithm>
#include <chrono>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

void printElements(vector<int> vec) {
  for (const auto elem : vec) {
    cout << elem << " ";
  }
  cout << endl;
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

void fillVector(int n, vector<int> &vec) {
  const int intervalUntil = 100;

  for (size_t i = 0; i < n; ++i) {
    vec.push_back(rand() % (intervalUntil + 1));
  }
}

int main() {
  const int elements = 100;
  vector<int> vec;

  fillVector(elements, vec);

  const chrono::time_point<chrono::system_clock> start =
      chrono::system_clock::now();

  idadeRep(vec);

  const chrono::time_point<chrono::system_clock> end =
      chrono::system_clock::now();

  const chrono::duration<long double> elapsed = end - start;

  cout << "Time elapsed for idadeRep: " << elapsed.count() << " seconds."
       << endl;

  return 0;
}
