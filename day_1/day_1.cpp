#include <cmath>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <vector>

struct Code {
  char dir;
  int val;
  Code(char dir, int val = 0) : dir(dir), val(val) {};
};

std::vector<Code> vec;

void readFile() {
  std::ifstream file("q1.txt");
  if (file.is_open()) {
    std::string line;
    while (file >> line) {
      if (line.empty()) continue;
      size_t n = line.size();
      Code cmd(line[0]);
      if (n > 1) {
        cmd.val = std::atoi(line.substr(1, n).c_str());
        vec.push_back(cmd);
      }
    }
  } else {
    std::cerr << "Unable to open file" << std::endl;
  }
  file.close();
}

int normalize(long val) {
    return ((val % 100) + 100) % 100;
}

int main() {
  readFile();

  long cnt = 0;
  long currVal = 50;

  for (size_t i = 0; i < vec.size(); i++) {
    int val = vec[i].val;
    
    if (vec[i].dir == 'R') {
        double start = (double)currVal;
        double end = (double)(currVal + val);
        cnt += (long)(std::floor(end / 100.0) - std::floor(start / 100.0));
        currVal += val;
    } else {
        double startAdjusted = (double)(currVal - 1);
        double endAdjusted = (double)(currVal - val - 1);
        cnt += (long)(std::floor(startAdjusted / 100.0) - std::floor(endAdjusted / 100.0));
        currVal -= val;
    }

    currVal = normalize(currVal);
  }

  std::cout << cnt << std::endl;
  return 0;
}
