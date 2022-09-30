#include <string.h>
#include <types.h>

void *
memcpy(void *restrict dst, const void *restrict src, size_t n)
{
	while (n--)
		((byte *) dst)[n] = ((byte *) src)[n];
	return dst;
}
