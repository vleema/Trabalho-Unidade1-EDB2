#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

#include "./parser.cpp"

using namespace std;
int random_index(int listSize){
  std::srand(std::time(0)); // Inicializa a semente para números aleatórios
  return std::rand() % listSize;
}

string format_time(long double time) {
  vector<string> prefixes{"m", "µ", "n", "p"}; // 10 ^ -3,-6,-9,-12
  size_t prefixes_index = 0;

  while (time < 1 && prefixes_index < prefixes.size()) {
    time *= 1000;
    prefixes_index++;
  }

  ostringstream oss;
  oss << time << prefixes[prefixes_index] << "s.";

  return oss.str();
}

long double get_average(vector<chrono::duration<long double>> times) {
  chrono::duration<long double> sum = chrono::duration<long double>(0);

  for (const auto &elapsed : times) {
    sum += elapsed;
  }

  return sum.count() / times.size();
}

template <typename T, typename... Args>
void generate_report(ofstream &output, vector<int> ns, string function_name,
                     T (*func)(Args...)) {
  const size_t how_many_lists = 5;

  // Header
  output << function_name << endl;

  for (const auto &n : ns) {
    vector<chrono::duration<long double>> times;

    output << "   n=" << n << endl;

    for (size_t i = 1; i <= how_many_lists; i++) {

      vector<int> list = parse_file(n, i);

      // Just initializing
      chrono::time_point<std::chrono::system_clock>  start = chrono::system_clock::now();
      chrono::time_point<std::chrono::system_clock>  end = chrono::system_clock::now();

      // Analysis
      if constexpr (sizeof...(Args) == 1) {
        start = chrono::system_clock::now();

        func(list); // idadeReps

        end = chrono::system_clock::now();
      } else if constexpr (sizeof...(Args) == 3) {
        // buscaBinaria(A,n,x)
        int n = list.size(); // A.size
        int x = list[random_index(n)]; // key

        sort(list.begin(), list.end());

        start = chrono::system_clock::now();

        func(list, n, x);

        end = chrono::system_clock::now();
      } else if constexpr (sizeof...(Args) == 4) {
        // bBinRec(A,esq,dir,x)

        int esq = 0;
        int dir = list.size();
        int x = list[random_index(dir)]; // key

        sort(list.begin(), list.end());
        start = chrono::system_clock::now();

        func(list, esq, dir, x);

        end = chrono::system_clock::now();
      }

      chrono::duration<long double> elapsed = end - start;
      times.push_back(elapsed);

      output << "      List-" << i << " done in "
             << format_time(elapsed.count()) << endl;
    }
    output << "   Average: " << format_time(get_average(times)) << endl << endl;
  }
  output << endl;
}
