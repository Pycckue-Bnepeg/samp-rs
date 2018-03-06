/*!
Some useful macros to easy access and define natives.

Most of them hide raw C bindings and exports to make code easier to understand.
*/

/// Clear macros that makes a new `Vec<AMX_NATIVE_INFO>`.
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

    [ $( $name:expr => $func:ident ),* ] => {
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

/// Hides ugly C code from your eyes.
/// 
/// Generates raw extern C functions and makes call to your own static methods. 
///
/// # Examples
///
/// ```
/// struct MyPlugin;
/// 
/// impl MyPlugin {
///     fn load(&self) -> bool {
///         amx_log!("My plugin is loaded!");
///         return true;
///     }
///     
///     fn unload(&self);
///     fn amx_load(&self, amx: AMX) -> Cell;
///     fn amx_unload(&self, amx: AMX) -> Cell;
/// }
///
/// impl Default for MyPlugin {
///     fn default() -> MyPlugin {
///         MyPlugin{}
///     }
/// }
/// 
/// new_plugin!(MyPlugin) 
/// ```
/// To make a plugin with `ProccessTick` support use this:
/// ```
/// impl MyPlugin {
///     fn process_tick(&self);
/// }
///
/// new_plugin!(MyPlugin with process_tick);
/// ```
#[macro_export]
macro_rules! new_plugin {
    (@internal $name:ident) => {
        lazy_static! {
            static ref ___PLUGIN: ::std::sync::Mutex<$name> = ::std::sync::Mutex::new($name::default());
        }

        #[no_mangle]
        pub unsafe extern "C" fn Load(data: *const *const u32) -> bool {
            let mut log = $crate::data::logprintf.lock().unwrap();

            *log = *(data as *const $crate::types::Logprintf_t);
            $crate::data::amx_functions = std::ptr::read(data.offset($crate::consts::PLUGIN_DATA_AMX_EXPORTS as isize) as *const *const u32);

            drop(log);
            ___PLUGIN.lock().unwrap().load()
        }

        #[no_mangle]
        pub extern "C" fn Unload() {
            ___PLUGIN.lock().unwrap().unload();
        }

        #[no_mangle]
        pub extern "C" fn AmxLoad(amx: *mut $crate::types::AMX) -> u32 {
            ___PLUGIN.lock().unwrap().amx_load($crate::amx::AMX::new(amx))
        }

        #[no_mangle]
        pub extern "C" fn AmxUnload(amx: *mut $crate::types::AMX) -> u32 {
            ___PLUGIN.lock().unwrap().amx_unload($crate::amx::AMX::new(amx))
        }
    };

    ($name:ident) => {
        new_plugin!(@internal $name);

        #[no_mangle]
        pub extern "C" fn Supports() -> u32 {
            $crate::consts::SUPPORTS_VERSION | $crate::consts::SUPPORTS_AMX_NATIVES
        }
    };

    ($name:ident with process_tick) => {
        new_plugin!(@internal $name);

        #[no_mangle]
        pub extern "C" fn ProcessTick() {
            ___PLUGIN.lock().unwrap().process_tick();
        }

        #[no_mangle]
        pub extern "C" fn Supports() -> u32 {
            $crate::consts::SUPPORTS_PROCESS_TICK | $crate::consts::SUPPORTS_VERSION | $crate::consts::SUPPORTS_AMX_NATIVES
        }
    }
}

/// Useful macro to log to SA:MP server output.
///
/// Take a look at println! in Rust Standard Library.
#[macro_export]
macro_rules! log {
    ($( $arg:tt )* ) => {
        {
            let printf = $crate::data::logprintf.lock().unwrap();
            let c_text = ::std::ffi::CString::new(format!($( $arg )*)).unwrap();
            printf(c_text.as_ptr());
        }
    }
}

/// Define native and hide raw C export functions.
///
/// # Examples
/// Define a native with raw params (`*mut Cell`).
/// ```
/// // native: WithRawParams(&arg1, arg2, arg3);
/// define_native!(with_raw_params as raw);
///
/// fn with_raw_params(&self, amx: AMX, args: *mut Cell) -> AmxResult<Cell>;
/// ```
///
/// Define a native without arguments.
/// ```
/// // native: WithoutArguments();
/// define_native!(without_arguments);
///
/// fn without_arguments(&self, amx: AMX) -> AmxResult<Cell>;
/// ```
/// 
/// Define a native with converted arguments.
/// ```
/// // native: SomeFunction(&int_val, float_val);
/// define_native!(some_function, int_val: ref i32, float_val: f32);
///
/// fn some_function(&self, amx: AMX, int_val: &mut i32, float_val: f32) -> AmxResult<Cell>;
/// ```
#[macro_export]
macro_rules! define_native {
    ($name:ident as raw) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            match super::___PLUGIN.lock().unwrap().$name(&mut amx, params) {
                Ok(res) => return res,
                Err(err) => {
                    amx.raise_error(err).unwrap();
                    return 0;
                },
            };
        }
    };

    ($name:ident) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, _: *mut $crate::types::Cell) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            match super::___PLUGIN.lock().unwrap().$name(&mut amx) {
                Ok(res) => return res,
                Err(err) => {
                    amx.raise_error(err).unwrap();
                    return 0;
                },
            };
        }
    };

    ($name:ident, $( $arg:ident : $( $data:ident )+ ),* ) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            expand_args!(amx, params, $( $arg : $( $data )+ ),* );
            
            let retval = super::___PLUGIN.lock().unwrap().$name(&mut amx, $( 
                    ___internal_expand_arguments!( 
                        $arg: $( $data )+ 
                    )
                ),* 
            );

            ___internal_forget!( $( $arg : $( $data )+ ),* );

            match retval {
                Ok(res) => return res,
                Err(err) => {
                    amx.raise_error(err).unwrap();
                    return 0;
                },
            };
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
    // A string.
    (
        @
        $amx:ident,
        $parser:ident,

        $arg:ident : String,
    ) => {
        let $arg = {
            let arg = $parser.next();
            match get_string!($amx, arg) {
                Ok(res) => res,
                Err(err) => {
                    $amx.raise_error(err).unwrap();
                    return 0;
                },
            }
        };
    };

    // TODO: A reference to a string.
    (
        @
        $amx:ident,
        $parser: ident,

        $arg:ident : ref String,
    ) => {
        unimplemented!();
    };

    // A reference to an primitive value.
    (
        @
        $amx:ident,
        $parser:ident,
        
        $arg:ident : ref $type:ty
    ) => {
        let mut $arg: Box<$type> = unsafe {
            let ptr = $parser.next();
            match $amx.get_address(::std::ptr::read(ptr as *const $crate::types::Cell)) {
                Ok(res) => res,
                Err(err) => {
                    $amx.raise_error(err).unwrap();
                    return 0;
                },
            }
        };
    };

    // An primitive value.
    (
        @
        $amx:ident,
        $parser:ident,

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

/// Get count of passed arguments in a native.
///
/// # Examples
/// ```
/// fn native(&self, _amx: AMX, params: *mut Cell) -> AmxResult<Cell> {
///     let count = args_count!(params);
///     log!("Args count: {}", count);
/// }
/// ```
#[macro_export]
macro_rules! args_count {
    ($params:ident) => {
        unsafe {
            ::std::ptr::read($params) as usize / ::std::mem::size_of::<$crate::types::Cell>()
        }
    }
}

/// Executes `AMX::exec` with given arguments.
///
/// # Examples
/// fn native(&self, amx: AMX) -> AmxResult<Cell> {
///     let public = amx.find_public("TestPublic");
///     let player_name = String::from("Some_Name");
///     let player_id = 12;
///     let player_data = vec![51, 2, 256, 0, 22];
///     let data_size = player_data.len();
///     
///     let res = exec!(amx, public; 
///         player_name => string, 
///         player_id, 
///         player_data => array, 
///         data_size
///     );
/// }
#[macro_export]
macro_rules! exec {
    (@internal
        $addr:ident,
        $amx:ident;
        $arg:expr
    ) => {
        $amx.push($arg)?;
    };

    (@internal
        $addr:ident,
        $amx:ident;
        $arg:expr => string
    ) => {
        let __res = $amx.push_string(&$arg, false)?;
        if $addr.is_none() {
            $addr = Some(__res);
        }
    };

    (@internal
        $addr:ident,
        $amx:ident;
        $arg:expr => array
    ) => {
        let __res = $amx.push_array(&$arg)?;
        if $addr.is_none() {
            $addr = Some(__res);
        }
    };

    (@internal
        $addr:ident,
        $amx:ident;
        $arg:ident,
        $($tail:tt)*
    ) => {
        exec!(@internal $addr, $amx; $($tail)*);
        exec!(@internal $addr, $amx; $arg);
    };

    (@internal
        $addr:ident,
        $amx:ident;
        $arg:ident => string,
        $($tail:tt)*
    ) => {
        exec!(@internal $addr, $amx; $($tail)*);
        exec!(@internal $addr, $amx; $arg => string);
    };

    (@internal
        $addr:ident,
        $amx:ident;
        $arg:ident => array,
        $($tail:tt)*
    ) => {
        exec!(@internal $addr, $amx; $($tail)*);
        exec!(@internal $addr, $amx; $arg => array);
    };

    (
        $amx:ident, 
        $index:ident;
        $($tail:tt)*
    ) => {
        {
            let mut __first_addr = None;
            exec!(@internal __first_addr, $amx; $($tail)*);
            let res = $amx.exec($index);
            if let Some(addr) = __first_addr {
                $amx.release(addr)?;
            }
            res
        }
    };
}

/// Finds a public and executes `AMX::exec` with given arguments.
///
/// # Examples
/// fn native(&self, amx: AMX) -> AmxResult<Cell> {
///     let old_name = String::from("Old_Name");
///     let new_name = String::from("Name_Surname");
///     exec_public!(amx, "OnPlayerNameChanged"; old_name => string, new_name => string); 
/// }
#[macro_export]
macro_rules! exec_public {
    ($amx:ident, $name:expr; $($args:tt)*) => {
        {
            $amx.find_public($name)
                .and_then(|index| exec!($amx, index; $($args)*))
        }
    };
}

/// Finds a native function and executes `AMX::exec` with given arguments.
///
/// # Examples
/// Same as `exec_public!`.
#[macro_export]
macro_rules! exec_native {
    ($amx:ident, $name:expr; $($args:tt)*) => {
        {
            $amx.find_native($name)
                .and_then(|index| exec!($amx, index; $($args)*))
        }
    }
}

/// Gets a string from `Cell`.
///
/// # Examples
/// // native:PushString(const string[]);
/// fn raw_arguments(&self, amx: AMX, args: *mut Cell) -> AmxResult<Cell> {
///     let string = get_string!(amx, args.offset(1));
///     log!("got a string: {}", string);
///     Ok(0)
/// }
#[macro_export]
macro_rules! get_string {
    ($amx:ident, $cell:expr) => {
        {
            let pointer = unsafe {
                ::std::ptr::read($cell)
            };

            $amx.get_address_experemental::<i32>(pointer)
                .and_then(|address| {
                    $amx.string_len(address)
                        .and_then(|len| $amx.get_string_experemental(address, len))
                })
        }
    }
}