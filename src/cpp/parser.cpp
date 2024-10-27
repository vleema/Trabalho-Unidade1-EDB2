#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

vector<int> parse_file(int file_elements_cout, size_t file_index) {
  ostringstream file_name;
  vector<int> list;

  // Loading data on memory
  file_name << "../inputs/n=" << file_elements_cout << "/list-" << file_index
            << ".txt";

  ifstream file(file_name.str());

  if (!file) {
    cerr << "Error when on file opening: " << file_name.str() << endl;
  }

  string line;
  while (getline(file, line)) {
    list.push_back(stoi(line));
  }

  return list;
}
