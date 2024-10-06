#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>

#include "cgranges.h"

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

  FILE* annotation = fopen(argv[1], "r");

  char *ctg = "2";
  char line[1024];
  char chr[1024];
  while(fgets(line, 1024, annotation)) {
    int32_t start, end;

    parse_bed(line, &start, &end);

    cr_add(cr, ctg, start, end, 0);
  }

  fclose(annotation);

  cr_index(cr);

  FILE* variant = fopen(argv[2], "r");
  int64_t n, *b = 0, max_b = 0;
  while(fgets(line, 1024, variant)) {
    int32_t start, end;

    parse_bed(line, &start, &end);

    n = cr_overlap(cr, "2", start, end, &b, &max_b);
    black_box(b);
  }

  free(b);

  fclose(variant);

  cr_destroy(cr);
}
