#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn sanity_check() {
        let pattern = "a|ab";
        let subject = "ab";
        let expected = "ab";

        let mut regex = MaybeUninit::uninit();
        let res = unsafe { minrx_regncomp(regex.as_mut_ptr(), pattern.len(), pattern.as_ptr(), 0) };
        assert_eq!(res, minrx_result_t_MINRX_REG_SUCCESS as _);
        let mut regex = unsafe { regex.assume_init() };

        let mut regmatch = MaybeUninit::uninit();
        let res = unsafe {
            minrx_regnexec(
                &raw mut regex,
                subject.len(),
                subject.as_ptr(),
                1,
                regmatch.as_mut_ptr(),
                0,
            )
        };
        assert_eq!(res, minrx_result_t_MINRX_REG_SUCCESS as _);
        let regmatch = unsafe { regmatch.assume_init() };

        assert_ne!(regmatch.rm_so, -1);
        let match_len = usize::try_from(regmatch.rm_eo - regmatch.rm_so).unwrap();
        let start = usize::try_from(regmatch.rm_so).unwrap();
        assert_eq!(&subject[start..start + match_len], expected);
    }
}
