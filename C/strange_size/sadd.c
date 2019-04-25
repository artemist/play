#include <stdio.h>
#include <stdint.h>

int main(int argc, char **argv) {
	uint32_t i = 1 << 31;
	uint32_t j = 3 << 30;
	uint32_t k = i + j;
	printf("%d + %d = %d\n", i, j, k);
	return 0;
}
