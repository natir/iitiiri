#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>

#include "cgranges/cgranges.h"

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

inline void black_box(void* value) {
  asm volatile("" : "+r,m"(value) : :);
}

int main(int argc, char* argv[]) {
  cgranges_t *cr = cr_init();

  FILE* reader = fopen(argv[1], "r");

  char *ctg = "2";
  char line[1024];
  char chr[1024];
  while(fgets(line, 1024, reader)) {
    int32_t start, end;

    parse_bed(line, &start, &end);

    cr_add(cr, ctg, start, end, 0);
  }

  fclose(reader);

  cr_index(cr);

  size_t start_min = 0;
  size_t start_max = 200000000;
  size_t length_max = 2000;
  size_t number_query = 1 << 9;

  for(size_t i = 0; i != number_query; i++) {
    size_t start = rand() % (start_max - start_min + 1) + start_min;
    size_t length = rand() % (2000 + 1);

    int64_t n, *b = 0, max_b = 0;

    struct timespec begin;
    struct timespec end;
    timespec_get(&begin, TIME_UTC);
    for (size_t i = 0; i != 100; i++) {
      n = cr_overlap(cr, "2", start, start + length, &b, &max_b);
      black_box(b);
    }
    timespec_get(&end, TIME_UTC);
    printf("cgranges,%i,%i\n", i, end.tv_nsec - begin.tv_nsec);

    free(b);
  }

  cr_destroy(cr);
}
