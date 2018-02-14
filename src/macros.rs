#[macro_export]
/// `natives!` macro
macro_rules! natives {
    [ $( { $name:expr, $func:ident } ),* ] => {
        {
            let natives = vec![
                $(
                    $crate::types::AMX_NATIVE_INFO {
                        name: ::std::ffi::CString::new($name).unwrap().as_ptr(),
                        func: $func,
                    }
                )*
            ];

            natives
        }
    };
}