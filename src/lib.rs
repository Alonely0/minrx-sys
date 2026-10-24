//! Raw FFI bindings to [MinRX](https://github.com/michaeljhaertel/minrx), by
//! Michael J. Haertel, which is vendored and compiled by this crate.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

/// Flags for `minrx_reg*comp()`.
pub type minrx_regcomp_flags_t = u32;
/// POSIX option: use "extended" syntax (required by this version of MinRX).
pub const minrx_regcomp_flags_t_MINRX_REG_EXTENDED: minrx_regcomp_flags_t = 1;
/// POSIX option: ignore case in pattern and search.
pub const minrx_regcomp_flags_t_MINRX_REG_ICASE: minrx_regcomp_flags_t = 2;
/// POSIX option: swap meaning of `?` and `??`, `*` and `*?`, `+` and `+?`.
pub const minrx_regcomp_flags_t_MINRX_REG_MINIMAL: minrx_regcomp_flags_t = 4;
/// POSIX option: exclude `\n` from `.` and `[^...]`; boundary for `^` and `$`.
pub const minrx_regcomp_flags_t_MINRX_REG_NEWLINE: minrx_regcomp_flags_t = 8;
/// POSIX option: yes/no result only; no substring results.
pub const minrx_regcomp_flags_t_MINRX_REG_NOSUB: minrx_regcomp_flags_t = 16;
/// Extension: `{` begins an interval expression only when followed by a digit.
pub const minrx_regcomp_flags_t_MINRX_REG_BRACE_COMPAT: minrx_regcomp_flags_t = 32;
/// Extension: bracket expressions allow backslash escapes.
pub const minrx_regcomp_flags_t_MINRX_REG_BRACK_ESCAPE: minrx_regcomp_flags_t = 64;
/// Extension: enable BSD extensions `\<` and `\>`.
pub const minrx_regcomp_flags_t_MINRX_REG_EXTENSIONS_BSD: minrx_regcomp_flags_t = 128;
/// Extension: enable GNU extensions `\b \B \s \S \w \W`.
pub const minrx_regcomp_flags_t_MINRX_REG_EXTENSIONS_GNU: minrx_regcomp_flags_t = 256;
/// Extension: use native encoding for 8-bit character sets.
pub const minrx_regcomp_flags_t_MINRX_REG_NATIVE1B: minrx_regcomp_flags_t = 512;
/// Extension: disable POSIX 2024 minimal repetitions.
pub const minrx_regcomp_flags_t_MINRX_REG_MINDISABLE: minrx_regcomp_flags_t = 1024;

/// Flags for `minrx_reg*exec()`.
pub type minrx_regexec_flags_t = u32;
/// POSIX option: don't match `^` at beginning of string.
pub const minrx_regexec_flags_t_MINRX_REG_NOTBOL: minrx_regexec_flags_t = 1;
/// POSIX option: don't match `$` at end of string.
pub const minrx_regexec_flags_t_MINRX_REG_NOTEOL: minrx_regexec_flags_t = 2;
/// Extension: repeated subexpressions capture their first occurrence.
pub const minrx_regexec_flags_t_MINRX_REG_FIRSTSUB: minrx_regexec_flags_t = 4;
/// Extension: repeated subexpressions don't clear their contained ones.
pub const minrx_regexec_flags_t_MINRX_REG_NOSUBRESET: minrx_regexec_flags_t = 8;
/// Extension: resume search from `rm[0].rm_eo`.
pub const minrx_regexec_flags_t_MINRX_REG_RESUME: minrx_regexec_flags_t = 16;
/// Extension: disable rapid skip-ahead over impossible first bytes.
pub const minrx_regexec_flags_t_MINRX_REG_NOFIRSTBYTES: minrx_regexec_flags_t = 32;

/// Return values from `minrx_reg*comp()` and `minrx_reg*exec()`.
pub type minrx_result_t = u32;
/// regcomp: compiled successfully; regexec: match found.
pub const minrx_result_t_MINRX_REG_SUCCESS: minrx_result_t = 0;
/// regcomp: bad pattern (generic).
pub const minrx_result_t_MINRX_REG_BADPAT: minrx_result_t = 1;
/// regcomp: invalid contents of `{}`.
pub const minrx_result_t_MINRX_REG_BADBR: minrx_result_t = 2;
/// regcomp: `? * +` or `{interval}` not preceded by a valid subpattern.
pub const minrx_result_t_MINRX_REG_BADRPT: minrx_result_t = 3;
/// regcomp: unbalanced `{`.
pub const minrx_result_t_MINRX_REG_EBRACE: minrx_result_t = 4;
/// regcomp: unbalanced `[`.
pub const minrx_result_t_MINRX_REG_EBRACK: minrx_result_t = 5;
/// regcomp: invalid collating element.
pub const minrx_result_t_MINRX_REG_ECOLLATE: minrx_result_t = 6;
/// regcomp: invalid character class name.
pub const minrx_result_t_MINRX_REG_ECTYPE: minrx_result_t = 7;
/// regcomp: invalid backslash.
pub const minrx_result_t_MINRX_REG_EESCAPE: minrx_result_t = 8;
/// regcomp: unbalanced `(`.
pub const minrx_result_t_MINRX_REG_EPAREN: minrx_result_t = 9;
/// regcomp: invalid range endpoint.
pub const minrx_result_t_MINRX_REG_ERANGE: minrx_result_t = 10;
/// regcomp: memory allocation failed.
pub const minrx_result_t_MINRX_REG_ESPACE: minrx_result_t = 11;
/// regcomp: invalid `\digit`.
pub const minrx_result_t_MINRX_REG_ESUBREG: minrx_result_t = 12;
/// regexec: match not found.
pub const minrx_result_t_MINRX_REG_NOMATCH: minrx_result_t = 13;
/// Unknown error code.
pub const minrx_result_t_MINRX_REG_UNKNOWN: minrx_result_t = 14;

/// A compiled regular expression (`minrx_regex_t`).
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct minrx_regex_t {
    pub re_regexp: *mut c_void,
    pub re_nsub: usize,
    pub re_compflags: minrx_regcomp_flags_t,
}

/// Start/end byte offsets of a (sub)match (`minrx_regmatch_t`); `-1` if unset.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct minrx_regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

unsafe extern "C" {
    /// Compile a NUL-terminated pattern; returns a `minrx_result_t`.
    pub fn minrx_regcomp(rx: *mut minrx_regex_t, regexp: *const c_char, flags: c_int) -> c_int;

    /// Compile a pattern of explicit length; returns a `minrx_result_t`.
    pub fn minrx_regncomp(
        rx: *mut minrx_regex_t,
        nregexp: usize,
        regexp: *const c_char,
        flags: c_int,
    ) -> c_int;

    /// Search a NUL-terminated string; returns a `minrx_result_t`.
    pub fn minrx_regexec(
        rx: *mut minrx_regex_t,
        string: *const c_char,
        nrm: usize,
        rm: *mut minrx_regmatch_t,
        flags: c_int,
    ) -> c_int;

    /// Search a string of explicit length; returns a `minrx_result_t`.
    pub fn minrx_regnexec(
        rx: *mut minrx_regex_t,
        nstring: usize,
        string: *const c_char,
        nrm: usize,
        rm: *mut minrx_regmatch_t,
        flags: c_int,
    ) -> c_int;

    /// Write a message for `err` into `errbuf`; returns the size needed.
    pub fn minrx_regerror(
        err: c_int,
        rx: *const minrx_regex_t,
        errbuf: *mut c_char,
        nerrbuf: usize,
    ) -> usize;

    /// Free a compiled expression.
    pub fn minrx_regfree(rx: *mut minrx_regex_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{MaybeUninit, size_of};

    #[test]
    fn layout() {
        // `void *`, `size_t`, then a 4-byte enum padded to pointer alignment.
        assert_eq!(
            size_of::<minrx_regex_t>(),
            (2 * size_of::<usize>() + 4).next_multiple_of(size_of::<usize>())
        );
        assert_eq!(size_of::<minrx_regmatch_t>(), 2 * size_of::<usize>());
    }

    #[test]
    fn sanity_check() {
        let pattern = "a|ab";
        let subject = "ab";
        let expected = "ab";

        let mut regex = MaybeUninit::uninit();
        let res = unsafe {
            minrx_regncomp(
                regex.as_mut_ptr(),
                pattern.len(),
                pattern.as_ptr().cast(),
                minrx_regcomp_flags_t_MINRX_REG_EXTENDED as c_int,
            )
        };
        assert_eq!(res as minrx_result_t, minrx_result_t_MINRX_REG_SUCCESS);
        let mut regex = unsafe { regex.assume_init() };

        let mut regmatch = MaybeUninit::uninit();
        let res = unsafe {
            minrx_regnexec(
                &raw mut regex,
                subject.len(),
                subject.as_ptr().cast(),
                1,
                regmatch.as_mut_ptr(),
                0,
            )
        };
        assert_eq!(res as minrx_result_t, minrx_result_t_MINRX_REG_SUCCESS);
        let regmatch = unsafe { regmatch.assume_init() };
        unsafe { minrx_regfree(&raw mut regex) };

        assert_ne!(regmatch.rm_so, -1);
        let start = usize::try_from(regmatch.rm_so).unwrap();
        let end = usize::try_from(regmatch.rm_eo).unwrap();
        assert_eq!(&subject[start..end], expected);
    }

    #[test]
    fn bad_pattern_reports_error() {
        let pattern = "a(";
        let mut regex = MaybeUninit::zeroed();
        let res = unsafe {
            minrx_regncomp(
                regex.as_mut_ptr(),
                pattern.len(),
                pattern.as_ptr().cast(),
                minrx_regcomp_flags_t_MINRX_REG_EXTENDED as c_int,
            )
        };
        assert_eq!(res as minrx_result_t, minrx_result_t_MINRX_REG_EPAREN);
    }
}
