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
#[macro_export]
macro_rules! natives {
    [ $( { $name:expr, $func:ident } ),* ] => {
        {
            let natives = vec![
                $(
                    $crate::types::AMX_NATIVE_INFO {
                        name: ::std::ffi::CString::new($name).unwrap().into_raw(),
                        func: $func,
                    }
                ),*
            ];

            natives
        }
    };
}

/// `new_plugin!` macro
/// Hide ugly C code from your eyes.
///
/// # Examples
///
/// ```
/// struct MyPlugin;
/// 
/// impl MyPlugin {
///     fn load() -> bool {
///         amx_log!("My plugin is loaded!");
///         return true;
///     }
///     ...
/// }
/// 
/// new_plugin!(MyPlugin)
/// ```
#[macro_export]
macro_rules! new_plugin {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn Supports() -> u32 {
            $name::supports()
        }

        #[no_mangle]
        pub unsafe extern "C" fn Load(data: *const ::std::os::raw::c_void) -> bool {
            let mut log = $crate::data::logprintf.lock().unwrap();

            *log = *(data as *const $crate::types::Logprintf_t);
            $crate::data::amx_functions = std::ptr::read((data as u32 + $crate::consts::PLUGIN_DATA_AMX_EXPORTS) as *const u32);

            drop(log);
            $name::load()
        }

        #[no_mangle]
        pub extern "C" fn Unload() {
            $name::unload();
        }

        #[no_mangle]
        pub extern "C" fn AmxLoad(amx: *mut $crate::types::AMX) -> u32 {
            $name::amx_load($crate::amx::AMX::new(amx))
        }

        #[no_mangle]
        pub extern "C" fn AmxUnload(amx: *mut $crate::types::AMX) -> u32 {
            $name::amx_unload($crate::amx::AMX::new(amx))
        }
    }
}

#[macro_export]
macro_rules! log {
    ($( $arg:tt ),* ) => {
        {
            let printf = $crate::data::logprintf.lock().unwrap();
            let c_text = ::std::ffi::CString::new(format!($( $arg ),*)).unwrap();
            printf(c_text.as_ptr());
        }
    }
}

#[macro_export]
macro_rules! define_native {
    ($plugin:ident, $name:ident) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            $plugin::$name($crate::amx::AMX::new(amx), params)
        }
    }
}