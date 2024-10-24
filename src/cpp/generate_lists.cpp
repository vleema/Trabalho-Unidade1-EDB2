#include <experimental/random>
#include <fstream>
#include <strstream>
#include <vector>

using namespace std;

void create_file_with_random_numbers(string file_name, size_t n, int start,
                                     int finish) {
  ofstream ofs;

  ofs.open(file_name);

  for (size_t i = 0; i < n; i++) {
    ofs << experimental::randint(start, finish) << std::endl;
  }

  ofs.close();
}

int main(int argc, char *argv[]) {
  const vector<size_t> ns{100, 1000, 10000, 100000};
  const int range_start = 1;
  const int range_end = 100;
  const size_t how_many_files = 5;

  for (const auto &n : ns) {
    for (size_t i = 1; i <= how_many_files; i++) {
      ostrstream oss;

      oss << "../inputs/n=" << n << "/list-" << i << ".txt";

      create_file_with_random_numbers(oss.str(), n, range_start, range_end);
    }
  }

  return 0;
}
