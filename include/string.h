#ifndef __KYROS_STRING_H_
#define __KYROS_STRING_H_

#include <cdefs.h>
#include <stddef.h>

__DECLS_BEGIN

void *memcpy(void *restrict, const void *restrict, size_t);
void *memmove(void *, const void *, size_t);
void *memset(void *, int, size_t);
size_t strlen(const char *);

__DECLS_END

#endif /* !__KYROS_STRING_H_ */
