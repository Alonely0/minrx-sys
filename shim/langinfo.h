#ifndef MINRX_SYS_LANGINFO_H
#define MINRX_SYS_LANGINFO_H

#include <string.h>
#include <wchar.h>

#define CODESET 14

/* Windows has no <langinfo.h>. minrx only asks for CODESET and only compares
 * the result to "UTF-8", so detect UTF-8 by decoding U+00E9 in the current
 * locale rather than parsing locale names. */
static inline const char *nl_langinfo(int item)
{
	mbstate_t st;
	wchar_t wc = 0;
	(void) item;
	memset(&st, 0, sizeof st);
	return (mbrtowc(&wc, "\xC3\xA9", 2, &st) == 2 && wc == 0xE9) ? "UTF-8" : "";
}

#endif
