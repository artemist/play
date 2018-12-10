#include <stdio.h>
#include <stdint.h>

int main(int argc, char **argv) {
	uint8_t i = 72;
	uint8_t j = 84;
	uint8_t k = i + j;
	printf("%d + %d = %d\n", i, j, k);
	return 0;
}
