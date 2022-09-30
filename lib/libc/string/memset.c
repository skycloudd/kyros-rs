#include <string.h>
#include <types.h>

void *
memset(void *s, int c, size_t n)
{
	byte *p = (byte *) s;
	while (n--)
		p[n] = c;
	return s;
}
