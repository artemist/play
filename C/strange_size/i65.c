#include <stdio.h>
#include <stdint.h>

int main(int argc, char **argv) {
	uint64_t i = 1UL << 63;
	uint64_t j = 1UL << 63;
	uint64_t k = (i + j) >> 1;
	printf("(%lu + %lu) >> 1 = %lu\n", i, j, k);
	return 0;
}
