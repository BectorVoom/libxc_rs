//! `extern_c_wrapper!` — uniform wrapper for every extern "C" entry point.

/// Wrap an extern "C" body in NULL-handle check + `catch_unwind` + errno set + i32 return.
///
/// Forms:
///   1. With handle: `extern_c_wrapper!(p, "fn_name", { body returning Result<i32, LibxcRsError> })`
///   2. No handle:   `extern_c_wrapper!(_, "fn_name", { body })`
///
/// The no-handle `(_, ...)` arm MUST come first: since Rust 1.59 the underscore
/// is a valid `:expr` fragment (destructuring-assignment grammar), so a `$p:expr`
/// arm placed first would greedily capture a literal `_` and expand to the
/// invalid `if _.is_null()`. Ordering the literal-`_` arm first keeps the
/// no-handle form working while a real pointer expression still falls through.
#[macro_export]
macro_rules! extern_c_wrapper {
    (_, $name:literal, $body:block) => {{ $crate::__extern_c_wrapper_body!($name, $body) }};
    ($p:expr, $name:literal, $body:block) => {{
        if $p.is_null() {
            $crate::compat::errno::set_error(
                $crate::compat::errno::LIBXC_RS_NULL_HANDLE,
                concat!($name, ": null xc_func_type pointer"),
            );
            return $crate::compat::errno::LIBXC_RS_NULL_HANDLE;
        }
        $crate::__extern_c_wrapper_body!($name, $body)
    }};
}

/// Internal: shared catch_unwind body for both forms.
#[doc(hidden)]
#[macro_export]
macro_rules! __extern_c_wrapper_body {
    ($name:literal, $body:block) => {{
        let result: ::std::result::Result<i32, $crate::LibxcRsError> =
            ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $body))
            .unwrap_or_else(|payload| {
                let msg = if let Some(s) = payload.downcast_ref::<&str>() { (*s).to_string() }
                          else if let Some(s) = payload.downcast_ref::<String>() { s.clone() }
                          else { "unknown panic in libxc_rs compat layer".to_string() };
                $crate::compat::errno::set_error(
                    $crate::compat::errno::LIBXC_RS_PANIC,
                    &format!("{}: panic — {}", $name, msg),
                );
                Err($crate::LibxcRsError::Panicked { message: msg })
            });
        match result {
            Ok(code) => code,
            Err(e) => {
                let code = $crate::compat::errno::discriminant(&e);
                $crate::compat::errno::set_error(code, &e.to_string());
                code
            }
        }
    }};
}

#[cfg(test)] mod tests {
    use crate::compat::errno;

    #[unsafe(no_mangle)]
    unsafe extern "C" fn __test_compat_panic_fn() -> i32 {
        crate::extern_c_wrapper!(_, "__test_compat_panic_fn", {
            panic!("test panic from wrapper");
            #[allow(unreachable_code)]
            Ok::<i32, crate::LibxcRsError>(0)
        })
    }

    #[test]
    fn catch_panic_returns_errno() {
        let code = unsafe { __test_compat_panic_fn() };
        assert_eq!(code, errno::LIBXC_RS_PANIC);
        assert_eq!(errno::xc_rs_last_error_code(), errno::LIBXC_RS_PANIC);
        unsafe {
            let p = errno::xc_rs_last_error_message();
            let s = std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
            assert!(s.contains("test panic from wrapper"), "got: {s}");
        }
    }
}
