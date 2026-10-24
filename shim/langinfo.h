/*
 * Minimal <langinfo.h> replacement for platforms that lack one (MSVC, MinGW).
 *
 * MinRX only needs `nl_langinfo(CODESET)` to tell whether the current
 * LC_CTYPE locale uses UTF-8. On Windows the locale name carries the code page
 * (e.g. "en_US.65001" or "en_US.UTF-8"), so inspect that.
 */
#ifndef MINRX_SYS_SHIM_LANGINFO_H
#define MINRX_SYS_SHIM_LANGINFO_H

#include <locale.h>
#include <string.h>

#define CODESET 14

static __inline const char *
nl_langinfo(int item)
{
	const char *loc;

	(void) item;
	loc = setlocale(LC_CTYPE, (const char *) 0);
	if (loc != (const char *) 0 &&
	    (strstr(loc, ".65001") != (const char *) 0 ||
	     strstr(loc, ".UTF-8") != (const char *) 0 ||
	     strstr(loc, ".utf8") != (const char *) 0 ||
	     strstr(loc, ".UTF8") != (const char *) 0))
		return "UTF-8";
	return "";
}

#endif /* MINRX_SYS_SHIM_LANGINFO_H */
