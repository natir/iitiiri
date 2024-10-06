#include <functional>
#include <fstream>
#include <random>
#include <string>
#include <iostream>
#include <chrono>

#include "iitii.h"

template <class Tp>
inline void black_box(Tp& value) {
  asm volatile("" : "+r,m"(value) : :);
}

using intpair = std::pair<int,int>;
int p_get_beg(const intpair& p) { return p.first; }
int p_get_end(const intpair& p) { return p.second; }
using p_iitii = iitii<int, intpair, p_get_beg, p_get_end>;  // first arg is position type

char *parse_bed(char *s, int32_t *st_, int32_t *en_)
{
	char *p, *q, *ctg = 0;
	int32_t i, st = -1, en = -1;
	for (i = 0, p = q = s;; ++q) {
		if (*q == ' ' || *q == '\0') {
			int c = *q;
			*q = 0;
			if (i == 0) ctg = p;
			else if (i == 1) st = atol(p);
			else if (i == 2) en = atol(p);
			++i, p = q + 1;
			if (c == '\0') break;
		}
	}
	*st_ = st, *en_ = en;
	return i >= 3? ctg : 0;
}

int main(int argc, char* argv[]) {
  p_iitii::builder br;

  std::ifstream reader(argv[1]);

  if(!reader.is_open()) throw std::runtime_error("Error in file opening");

  std::string line;
  while(std::getline(reader, line)) {
    int32_t start, end;

    parse_bed(line.data(), &start, &end);

    br.add(intpair(start, end));
  }

  size_t domain = std::stoi(argv[2]);
  p_iitii db = br.build(domain);

  std::mt19937_64 rng;
  rng.seed(42);
  std::uniform_int_distribution<> start_gen(0, 200000000);
  std::uniform_int_distribution<> length_gen(0, 2000);
  size_t number_query = 1 << 9;
  std::vector<const intpair*> results;

  for(size_t i = 0; i != number_query; i++) {
    size_t start = start_gen(rng);
    size_t length = length_gen(rng);

    std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
    for (size_t i = 0; i != 100; i++) {
      db.overlap(start, start + length, results);
      black_box(results);
    }
    std::cout<<"iitii_"<<domain<<","<<i<<","<<std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now() - begin).count()<<std::endl;
  }
}
