#include <string.h>
#include <types.h>

void *
memmove(void *s1, const void *s2, size_t n)
{
	byte *dst = s1;
	byte *src = s2;

	if (src > dst) {
		for (size_t i = 0; i < n; i++)
			s1[i] = s2[i];
	} else {
		while (n--)
			s1[n] = s2[n];
	}

	return s1;
}
