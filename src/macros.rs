/*!
Some useful macros to easy access and define natives.

Most of them hide raw C bindings and exports to make code easier to understand.
*/

/// Clear macros that makes a new `Vec<AMX_NATIVE_INFO>`.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::amx::AMX;
/// use samp_sdk::types;
///
/// fn amx_load(amx: &AMX) {
///     extern "C" fn show_something(_: *mut types::AMX, _: *mut i32) -> i32 { 0 }
///     extern "C" fn where_is_player(_: *mut types::AMX, _: *mut i32) -> i32 { 0 }
///
///     let natives = natives![
///        { "ShowSomething", show_something },
///        { "WhereIsPlayer", where_is_player }
///     ];
///     amx.register(&natives);
/// }
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
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::amx::AMX;
/// use samp_sdk::types;
///
/// struct MyPlugin;
///
/// impl MyPlugin {
///     fn load(&self) -> bool {
///         log!("My plugin is loaded!");
///         true
///     }
///
///     fn unload(&self) {}
///     fn amx_load(&self, amx: &AMX) -> types::Cell { 0 }
///     fn amx_unload(&self, amx: &AMX) -> types::Cell { 0 }
/// }
///
/// impl Default for MyPlugin {
///     fn default() -> MyPlugin {
///         MyPlugin {}
///     }
/// }
///
/// new_plugin!(MyPlugin) ;
/// ```
///
/// To make a plugin with `ProccessTick` support use this:
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::amx::AMX;
/// use samp_sdk::types;
///
/// struct MyPlugin;
///
/// impl MyPlugin {
///     fn process_tick(&self) {}
///
///     fn load(&self) -> bool { true }
///     fn unload(&self) {}
///     fn amx_load(&self, amx: &AMX) -> types::Cell { 0 }
///     fn amx_unload(&self, amx: &AMX) -> types::Cell { 0 }
/// }
///
/// impl Default for MyPlugin {
///     fn default() -> MyPlugin {
///         MyPlugin {}
///     }
/// }
///
/// new_plugin!(MyPlugin with process_tick);
/// ```
#[macro_export]
macro_rules! new_plugin {
    (@internal $name:ident) => {
        // lazy_static! {
        //     pub static ref ___PLUGIN: ::std::sync::Mutex<$name> = ::std::sync::Mutex::new($name::default());
        // }

        pub static mut ___PLUGIN: *mut $name = 0 as *mut $name;

        #[no_mangle]
        pub unsafe extern "system" fn Load(data: *const *const u32) -> bool {
            let mut log = $crate::data::logprintf.lock().unwrap();

            ___PLUGIN = Box::into_raw(Box::new($name::default()));

            *log = *(data as *const $crate::types::Logprintf_t);
            $crate::data::amx_functions = std::ptr::read(data.offset($crate::consts::PLUGIN_DATA_AMX_EXPORTS as isize) as *const *const u32);

            drop(log);
            (*___PLUGIN).load()
        }

        #[no_mangle]
        pub unsafe extern "system" fn Unload() {
            (*___PLUGIN).unload();
        }

        #[no_mangle]
        pub unsafe extern "system" fn AmxLoad(amx: *mut $crate::types::AMX) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            (*___PLUGIN).amx_load(&mut amx)
        }

        #[no_mangle]
        pub unsafe extern "system" fn AmxUnload(amx: *mut $crate::types::AMX) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            (*___PLUGIN).amx_unload(&mut amx)
        }
    };

    ($name:ident) => {
        new_plugin!(@internal $name);

        #[no_mangle]
        pub extern "system" fn Supports() -> u32 {
            $crate::consts::SUPPORTS_VERSION | $crate::consts::SUPPORTS_AMX_NATIVES
        }
    };

    ($name:ident with process_tick) => {
        new_plugin!(@internal $name);

        #[no_mangle]
        pub unsafe extern "system" fn ProcessTick() {
            (*___PLUGIN).process_tick();
        }

        #[no_mangle]
        pub extern "system" fn Supports() -> u32 {
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
/// ```compile_fail
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::amx::{AMX, AmxResult};
/// use samp_sdk::types::Cell;
///
/// // native: WithRawParams(&arg1, arg2, arg3);
/// define_native!(with_raw_params as raw);
///
/// fn with_raw_params(amx: &AMX, args: *mut Cell) -> AmxResult<Cell> { Ok(0) };
/// ```
///
/// Define a native without arguments.
/// ```compile_fail
/// // native: WithoutArguments();
/// define_native!(without_arguments);
///
/// fn without_arguments(&self, amx: &AMX) -> AmxResult<Cell>;
/// ```
///
/// Define a native with converted arguments.
/// ```compile_fail
/// // native: SomeFunction(&int_val, float_val);
/// define_native!(some_function, int_val: ref i32, float_val: f32);
///
/// fn some_function(&self, amx: &AMX, int_val: &mut i32, float_val: f32) -> AmxResult<Cell>;
/// ```
#[macro_export]
macro_rules! define_native {
    ($name:ident as raw) => {
        pub extern "C" fn $name(amx: *mut $crate::types::AMX, params: *mut $crate::types::Cell) -> $crate::types::Cell {
            let mut amx = $crate::amx::AMX::new(amx);
            match unsafe { (*crate::___PLUGIN).$name(&mut amx, params) } {
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
            match unsafe { (*crate::___PLUGIN).$name(&mut amx) } {
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

            let retval = unsafe {
                (*crate::___PLUGIN).$name(&mut amx, $($arg),*)
            };

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
macro_rules! expand_args {
    // A string.
    (
        @
        $amx:ident,
        $parser:ident,

        $arg:ident : String
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

    // A reference to an primitive value.
    (
        @
        $amx:ident,
        $parser:ident,

        $arg:ident : ref $type:ty
    ) => {
        let $arg: &mut $type = unsafe {
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

        $arg:ident : String,
        $( $tail_arg:ident : $( $tail_data:ident )+ ),*
    ) => {
        expand_args!(@$amx, $parser, $arg : String);
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
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::types::Cell;
/// use samp_sdk::amx::{AMX, AmxResult};
///
/// fn native(_amx: &AMX, params: *mut Cell) -> AmxResult<Cell> {
///     let count = args_count!(params);
///     log!("Args count: {}", count);
///     Ok(1)
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

// FIXME: describe more clearly syntax of exec macro
// FIXME: write valid code example
/// Executes `AMX::exec` with given arguments.
///
/// # Examples
/// ```compile_fail
///
/// fn native(&self, amx: &AMX) -> AmxResult<Cell> {
///     let public = amx.find_public("TestPublic");
///     let player_name = String::from("Some_Name");
///     let player_id = 12 as i32;
///     let player_data = vec![51, 2, 256, 0, 22];
///     let data_size = player_data.len() as i32;
///
///     let res = exec!(
///         amx, public;
///
///         player_name => string,
///         player_id,
///         player_data => array,
///         data_size
///     );
/// }
/// ```
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
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::types::Cell;
/// use samp_sdk::amx::{AMX, AmxResult};
///
/// fn native(amx: &AMX) {
///     let old_name = String::from("Old_Name");
///     let new_name = String::from("Name_Surname");
///     exec_public!(amx, "OnPlayerNameChanged"; old_name => string, new_name => string);
/// }
/// ```
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
    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $arg:expr
    ) => {
        $params.push(unsafe {
            ::std::mem::transmute_copy(&$arg)
        });

        $count += 1;
    };

    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $head:expr,
        $($tail:tt)*
    ) => {
        exec_native!(@internal $amx, $params, $count, $addr; $head);
        exec_native!(@internal $amx, $params, $count, $addr; $($tail)*);
    };

    // strings
    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $arg:expr => string
    ) => {
        let bytes = $crate::cp1251::encode($arg)?;

        let (__amx, __phys) = $amx.allot(bytes.len() + 1)?;

        set_string!(bytes, __phys, bytes.len());
        $params.push(__amx);

        if $addr.is_none() {
            $addr = Some(__amx);
        }

        $count += 1;
    };

    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $head:expr => string,
        $($tail:tt)*
    ) => {
        exec_native!(@internal $amx, $params, $count, $addr; $head => string);
        exec_native!(@internal $amx, $params, $count, $addr; $($tail)*);
    };

    // arrays
    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $arg:expr => array
    ) => {
        let (__amx, __phys) = $amx.allot($arg.len())?;
        let __dest = __phys as *mut $crate::types::Cell;

        for i in 0..$arg.len() {
            unsafe {
                *(__dest.offset(i as isize)) = ::std::mem::transmute_copy(&$arg[i]);
            }
        }

        $params.push(__amx);

        if $addr.is_none() {
            $addr = Some(__amx);
        }

        $count += 1;
    };

    (@internal
        $amx:ident,
        $params:ident,
        $count:ident,
        $addr:ident;
        $head:expr => array,
        $($tail:tt)*
    ) => {
        exec_native!(@internal $amx, $params, $count, $addr; $head => array);
        exec_native!(@internal $amx, $params, $count, $addr; $($tail)*);
    };

    (@internal
        $amx:ident,
        $index:ident,
        $params:ident,
        $count:ident,
        $($args:tt)*
    ) => {
        {
            let mut __first_addr = None;

            exec_native!(@internal $amx, $params, $count, __first_addr; $($args)*);
            
            $params[0] = $count * ::std::mem::size_of::<$crate::types::Cell>() as $crate::types::Cell;

            let __native_addr = $amx.get_native_addr($index)?;
            let __func: $crate::types::AmxNative = unsafe { ::std::mem::transmute(__native_addr) };

            let __res = __func($amx.amx, $params.as_slice().as_ptr() as _);
            
            if let Some(__addr) = __first_addr {
                $amx.release(__addr)?;
            }
            
            Ok(__res)
        }
    };

    ($amx:ident, $name:expr; $($args:tt)*) => {
        {
            $amx.find_native($name)
                .and_then(|__index| {
                    let mut __params: Vec<$crate::types::Cell> = Vec::new();
                    let mut __count = 0;
                    
                    __params.push(__count);

                    exec_native!(@internal $amx, __index, __params, __count, $($args)*)
                })
        }
    }
}

/// Gets a string from a raw pointer to `Cell`.
///
/// Should used in `define_native!` and raw functions.
///
/// # Examples
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::types::Cell;
/// use samp_sdk::amx::{AMX, AmxResult};
///
/// // native:PushString(const string[]);
/// fn raw_arguments(amx: &AMX, args: *mut Cell) -> AmxResult<Cell> {
///     let string = get_string!(amx, args.offset(1))?;
///     log!("got a string: {}", string);
///     Ok(0)
/// }
/// ```
#[macro_export]
macro_rules! get_string {
    ($amx:ident, $cell:expr) => {
        {
            let pointer = unsafe {
                ::std::ptr::read($cell)
            };

            $amx.get_address::<i32>(pointer)
                .and_then(|address| {
                    $amx.string_len(address)
                        .and_then(|len| $amx.get_string(address, len))
                })
        }
    }
}

/// Get a slice (an array) from arguments.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::types::Cell;
/// use samp_sdk::amx::{AMX, AmxResult};
///
/// // native:PassArray(const array[], size);
/// // define_native!(pass_array, array_ptr: Cell, size: usize);
///
/// fn pass_array(amx: &AMX, array_ptr: Cell, size: usize) {
///     let array: &[u32] = get_array!(amx, array_ptr, size).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! get_array {
    ($amx:ident, $addr:expr, $len:expr) => {
        $amx.get_address($addr)
            .map(|pointer| unsafe { ::std::slice::from_raw_parts_mut(pointer, $len) })
    };
}

/// Sets a string to physical address.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate samp_sdk;
/// use samp_sdk::types::Cell;
/// use samp_sdk::amx::{AMX, AmxResult};
///
/// // native: rot13(const source[], dest[], size=sizeof(dest));
/// // define_native!(n_rot13, source: String, dest_ptr: &mut types::Cell, size: usize);
///
/// fn n_rot13(amx: &AMX, source: String, dest_ptr: &mut Cell, size: usize) -> AmxResult<Cell> {
///     let roted = rot13(source);
///     let encoded = samp_sdk::cp1251::encode(roted)?;
///     set_string!(encoded, dest_ptr, size);
///     Ok(0)
/// }
///
/// fn rot13(string: String) -> String {
///      let alphabet = [
///          'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
///          'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
///      ];
///
///      string.chars()
///            .map(|c| *alphabet.iter()
///                              .chain(alphabet.iter())
///                              .skip_while(|&x| *x != c)
///                              .nth(13)
///                              .unwrap_or(&c))
///            .collect()
/// }
/// ```
#[macro_export]
macro_rules! set_string {
    ($string:expr, $address:expr, $size:expr) => {
        {
            let length = if $string.len() > $size { $size } else { $string.len() };
            let dest = $address as *mut $crate::types::Cell;

            for i in 0..length {
                unsafe {
                    *(dest.offset(i as isize)) = $string[i] as i32;
                }
            }

            unsafe {
                *(dest.offset(length as isize)) = 0;
            }
        }
    }
}