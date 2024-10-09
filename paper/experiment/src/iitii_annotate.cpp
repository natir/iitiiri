#include <functional>
#include <fstream>
#include <string>

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

  std::ifstream annotation(argv[1]);
  std::ifstream variant(argv[2]);

  if(!annotation.is_open()) throw std::runtime_error("Error in file opening");

  std::string line;
  while(std::getline(annotation, line)) {
    int32_t start, end;

    parse_bed(line.data(), &start, &end);

    br.add(intpair(start, end));
  }
  size_t domain = std::stoi(argv[3]);
  p_iitii db = br.build(domain);

  std::vector<const intpair*> results;
  while(std::getline(variant, line)) {
    int32_t start, end;

    parse_bed(line.data(), &start, &end);

    db.overlap(start, end, results);
    black_box(results);
  }
}
