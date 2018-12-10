#include <stdio.h>
#include <stdint.h>

int main(int argc, char **argv) {
	uint64_t i = (uint64_t)(-1LL);
	uint64_t j = (uint64_t)(-1LL);
	uint64_t k = i + j;
	printf("%lu + %lu = %lu\n", i, j, k);
	return 0;
}
