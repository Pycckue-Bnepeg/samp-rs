#[macro_export]
/// `natives!` macro
///
/// # Examples
///
/// ```
/// let natives = natives![
///    { "ShowSomething", show_something },
///    { "WhereIsPlayer", where_is_player }
/// ];
/// amx.register(natives);
/// ```
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