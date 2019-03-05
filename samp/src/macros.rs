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
