/// Execute a public AMX function by name.
///
/// # Notes
/// Function input arguments should implement `AmxCell` except *Rust* strings and slices.
///
/// To pass a Rust string there is the next syntax - `variable_name => string`, for array `variable_name => array`.
///
/// In this case inside the macro memory will be allocated for them and auto-released when the public will be executed.
///
/// # Examples
/// Simple execution.
/// ```rust,no_run
/// use samp_sdk::exec_public;
/// # use samp_sdk::amx::Amx;
/// #
/// # let amx_owned = Amx::new(std::ptr::null_mut(), 0);
/// # let amx = &amx_owned;
///
/// exec_public!(amx, "SomePublicFunction");
/// ```
///
/// With arguments that implement `AmxCell`.
/// ```rust,no_run
/// use samp_sdk::exec_public;
/// # use samp_sdk::amx::Amx;
/// # use samp_sdk::cell::{AmxString, UnsizedBuffer, Ref};
/// # use samp_sdk::error::AmxResult;
///
/// // native:CallPublic(const publicname[], const string[], buffer[], length, &someref);
/// fn call_public(amx: &Amx, pub_name: AmxString, string: AmxString, buffer: UnsizedBuffer, size: usize, reference: Ref<usize>) -> AmxResult<bool> {
///     let buffer = buffer.into_sized_buffer(size);
///     let public_name = pub_name.to_string();
///
///     exec_public!(amx, &public_name, string, buffer, reference);
///     Ok(true)
/// }
/// ```
/// And with Rust strings and slices.
/// ```rust,no_run
/// use samp_sdk::exec_public;
/// # use samp_sdk::amx::Amx;
/// # use samp_sdk::cell::{AmxString, UnsizedBuffer, Ref};
/// # use samp_sdk::error::AmxResult;
///
/// // native:CallPublic(const publicname[], const string[]);
/// fn call_public(amx: &Amx, pub_name: AmxString, string: AmxString) -> AmxResult<bool> {
///     let public_name = pub_name.to_string();
///     let rust_string = "hello!";
///     let owned_string = "another hello!".to_string();
///     let rust_array = vec![1, 2, 3, 4, 5];
///
///     exec_public!(amx, &public_name, string, rust_string => string, &owned_string => string, &rust_array => array);
///     Ok(true)
/// }
/// ```
#[macro_export]
macro_rules! exec_public {
    ($amx:expr, $pubname:expr) => {
        $amx.find_public($pubname)
            .and_then(|idx| $amx.exec(idx))
    };

    (@ $amx:expr, $al:ident, $arg:expr) => {
        $amx.push($arg)?;
    };

    (@ $amx:expr, $al:ident, $arg:expr, $($tail:tt)+) => {
        exec_public!(@ $amx, $al, $($tail)+);
        exec_public!(@ $amx, $al, $arg);
    };

    (@ $amx:expr, $al:ident, $arg:expr => string) => {
        let string = $al.allot_string($arg)?;
        $amx.push(string)?;
    };

    (@ $amx:expr, $al:ident, $arg:expr => string, $($tail:tt)+) => {
        exec_public!(@ $amx, $al, $($tail)+);
        exec_public!(@ $amx, $al, $arg => string);
    };

    (@ $amx:expr, $al:ident, $arg:expr => array) => {
        let array = $al.allot_array($arg)?;
        $amx.push(array)?;
    };

    (@ $amx:expr, $al:ident, $arg:expr => array, $($tail:tt)+) => {
        exec_public!(@ $amx, $al, $($tail)+);
        exec_public!(@ $amx, $al, $arg => array);
    };

    ($amx:expr, $pubname:expr, $($args:tt)+) => {
        {
            let allocator = $amx.allocator();
            $amx.find_public($pubname)
                .and_then(|idx| {
                    exec_public!(@ $amx, allocator, $($args)+);
                    $amx.exec(idx)
                })
        }
    };
}
