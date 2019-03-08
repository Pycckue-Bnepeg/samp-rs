//! Types to get exported functions by AMX.
use crate::raw::functions;

macro_rules! impl_export {
    ($name:ident) => {
        pub struct $name;

        impl Export for $name {
            type Output = functions::$name;
            const OFFSET: isize = Exports::$name as isize;

            #[inline(always)]
            fn from_table(fn_table: usize) -> Self::Output {
                let table = fn_table as *const usize;

                unsafe {
                    let ptr = table.offset(Self::OFFSET);
                    (ptr as *const Self::Output).read()
                }
            }
        }
    };
}

pub trait Export {
    type Output;
    const OFFSET: isize;

    fn from_table(fn_table: usize) -> Self::Output;
}

impl_export!(Align16);
impl_export!(Align32);
impl_export!(Allot);
impl_export!(Callback);
impl_export!(Cleanup);
impl_export!(Clone);
impl_export!(Exec);
impl_export!(FindNative);
impl_export!(FindPublic);
impl_export!(FindPubVar);
impl_export!(FindTagId);
impl_export!(Flags);
impl_export!(GetAddr);
impl_export!(GetNative);
impl_export!(GetPublic);
impl_export!(GetPubVar);
impl_export!(GetString);
impl_export!(GetTag);
impl_export!(GetUserData);
impl_export!(Init);
impl_export!(InitJIT);
impl_export!(MemInfo);
impl_export!(NameLength);
impl_export!(NativeInfo);
impl_export!(NumNatives);
impl_export!(NumPublics);
impl_export!(NumPubVars);
impl_export!(NumTags);
impl_export!(Push);
impl_export!(PushArray);
impl_export!(PushString);
impl_export!(RaiseError);
impl_export!(Register);
impl_export!(Release);
impl_export!(SetCallback);
impl_export!(SetDebugHook);
impl_export!(SetString);
impl_export!(SetUserData);
impl_export!(StrLen);
impl_export!(UTF8Check);
impl_export!(UTF8Get);
impl_export!(UTF8Len);
impl_export!(UTF8Put);

/// List of `amx_*` functions exported via the extern `Load` function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Exports {
    Align16 = 0,
    Align32 = 1,
    Align64 = 2,
    Allot = 3,
    Callback = 4,
    Cleanup = 5,
    Clone = 6,
    Exec = 7,
    FindNative = 8,
    FindPublic = 9,
    FindPubVar = 10,
    FindTagId = 11,
    Flags = 12,
    GetAddr = 13,
    GetNative = 14,
    GetPublic = 15,
    GetPubVar = 16,
    GetString = 17,
    GetTag = 18,
    GetUserData = 19,
    Init = 20,
    InitJIT = 21,
    MemInfo = 22,
    NameLength = 23,
    NativeInfo = 24,
    NumNatives = 25,
    NumPublics = 26,
    NumPubVars = 27,
    NumTags = 28,
    Push = 29,
    PushArray = 30,
    PushString = 31,
    RaiseError = 32,
    Register = 33,
    Release = 34,
    SetCallback = 35,
    SetDebugHook = 36,
    SetString = 37,
    SetUserData = 38,
    StrLen = 39,
    UTF8Check = 40,
    UTF8Get = 41,
    UTF8Len = 42,
    UTF8Put = 43,
}

impl From<Exports> for isize {
    fn from(exports: Exports) -> isize {
        exports as isize
    }
}
