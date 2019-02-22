use samp::{initialize_plugin, SampPlugin, amx::Amx};
use samp::cell::{Ref, Cell, UnsizedBuffer, AmxString};
use samp::error::AmxResult;
use samp::raw::types::AMX;

mod natives;

#[derive(Debug, Clone, Copy)]
enum NativeResult {
    AllGood,
    NoMana,
    NoHealth,
}

impl From<NativeResult> for i32 {
    fn from(value: NativeResult) -> i32 {
        match value {
            NativeResult::AllGood => 0,
            NativeResult::NoMana => 1,
            NativeResult::NoHealth => 2,
        }
    }
}

impl Cell<'_> for NativeResult {
    fn as_cell(&self) -> i32 {
        (*self).into()
    }
}

struct Plugin {
    admin_list: Vec<&'static str>,
}

impl Plugin {
    fn new(admin_list: Vec<&'static str>) -> Plugin {
        Plugin {
            admin_list,
        }
    }

    fn callback(amx: &Amx, ptr: Ref<usize>, somevalue: usize) -> AmxResult<NativeResult> {
        let allocator = amx.allocator();
        let val = allocator.allot::<f32>(1.0)?;

        let array = vec![1, 2, 3, 4, 5];

        let arr = allocator.allot_array(&array)?;
        let buf = allocator.allot_string("Hello, how are you?")?;

        amx.push(val)?;
        amx.push(10.0)?;
        amx.push(500)?;
        amx.push(&buf)?;
        amx.push(ptr)?;
        amx.push(somevalue)?;
        amx.push(&arr)?;

        if somevalue == 255 {
            return Ok(NativeResult::NoMana);
        }

        Ok(NativeResult::AllGood)
    }

    unsafe fn raw_callback(amx: *mut AMX, args: *mut i32) -> i32 {
        let amx = Amx::new(amx, 0);
        let ptr = Ref::<usize>::from_raw(&amx, args.offset(1).read()).unwrap();
        let somevalue = usize::from_raw(&amx, args.offset(2).read()).unwrap();
        let _ = UnsizedBuffer::from_raw(&amx, args.offset(3).read()).unwrap();
        let _ = AmxString::from_raw(&amx, args.offset(4).read()).unwrap();

        match Plugin::callback(&amx, ptr, somevalue) {
            Ok(retval) => convert(retval),
            Err(_) => 0,
        }
    }
}

fn convert<T: Cell<'static>>(value: T) -> i32 {
    value.as_cell()
} 

impl SampPlugin for Plugin {}

initialize_plugin!(
    natives: [
        Plugin::get_player_ip, 
        Plugin::is_player_admin,
    ],
    {
        samp::enable_process_tick();

        let admin_list = vec!["A", "B"];
        let plugin = Plugin::new(admin_list);
        unsafe { Plugin::raw_callback(std::ptr::null_mut(), std::ptr::null_mut()); }
        return plugin;
    }
);