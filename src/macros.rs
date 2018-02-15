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
    ($plugin:ident, $name:ident as raw) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            $plugin::$name($crate::amx::AMX::new(amx), params)
        }
    };

    ($plugin:ident, $name:ident) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, _: *mut $crate::types::Cell) -> $crate::types::Cell {
            $plugin::$name($crate::amx::AMX::new(amx))
        }
    };

    ($plugin:ident, $name:ident, $( $arg:ident : $( $data:ident )+ ),* ) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            let amx = $crate::amx::AMX::new(amx);
            expand_args!(amx, params, $( $arg : $( $data )+ ),* );
            
            let retval = $plugin::$name(amx, $( 
                    ___internal_expand_arguments!( 
                        $arg: $( $data )+ 
                    )
                ),* 
            );

            ___internal_forget!( $( $arg : $( $data )+ ),* );

            retval
        }
    }
}

#[macro_export]
macro_rules! ___internal_forget {
    (
        $arg:ident : ref $type:ty
    ) => {
        ::std::mem::forget($arg);
    };

    (
        $arg:ident : $type:ty
    ) => ();

    (
        $arg:ident : ref $type:ty,
        $( $tail_arg:ident : $( $tail_data:ident )+ ),*
    ) => {
        ___internal_forget!( $arg : ref $type );
        ___internal_forget!( $( $tail_arg : $( $tail_data )+ ),* );
    };

    (
        $arg:ident : $type:ty,
        $( $tail_arg:ident : $( $tail_data:ident )+ ),*
    ) => {
        ___internal_forget!( $( $tail_arg : $( $tail_data )+ ),* );
    };
}

#[macro_export]
macro_rules! ___internal_expand_arguments {
    (
        $arg:ident : ref $type:ty
    ) => {
        $arg.as_mut()
    };

    (
        $arg:ident : $type:ty
    ) => {
        $arg
    };
}

#[macro_export]
macro_rules! expand_args {
    (
        @
        $amx:ident,
        $parser:ident,
        
        // last ref arg
        $arg:ident : ref $type:ty
    ) => {
        let mut $arg: Box<$type> = unsafe {
            let ptr = $parser.next();
            $amx.get_address(::std::ptr::read(ptr as *const $crate::types::Cell)).unwrap()
        };
    };

    (
        @
        $amx:ident,
        $parser:ident,

        // last arg
        $arg:ident : $type:ty
    ) => {
        let $arg: $type = unsafe {
            let ptr = $parser.next();
            ::std::ptr::read(ptr as *const $type)
        };
    };

    (
        @
        $amx:ident,
        $parser:ident,

        $arg:ident : ref $type:ty,
        $( $tail_arg:ident : $( $tail_data:ident )+ ),*
    ) => {
        expand_args!(@$amx, $parser, $arg : ref $type);
        expand_args!(@$amx, $parser, $( $tail_arg : $( $tail_data )+ ),*);
    };

    (
        @
        $amx:ident,
        $parser:ident,

        $arg:ident : $type:ty,
        $( $tail_arg:ident : $( $tail_data:ident )+ ),*
    ) => {
        expand_args!(@$amx, $parser, $arg : $type);
        expand_args!(@$amx, $parser, $( $tail_arg : $( $tail_data )+ ),*);
    };

    (
        $amx:ident,
        $params:ident,

        $( $arg:ident : $($data:ident)+ ),*
    ) => {
        let mut parser = $crate::args::Parser::new($params);
        expand_args!(@$amx, parser, $( $arg : $( $data )+ ),*);
    };
}