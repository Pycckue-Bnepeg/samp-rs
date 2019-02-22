use samp::{initialize_plugin, SampPlugin, amx::Amx};
use samp::cell::{Ref, Cell, UnsizedBuffer};
use samp::error::AmxResult;
use samp::raw::types::AMX;

mod natives;

struct Plugin {
    admin_list: Vec<&'static str>,
}

impl Plugin {
    fn new(admin_list: Vec<&'static str>) -> Plugin {
        Plugin {
            admin_list,
        }
    }

    fn callback(amx: &Amx, ptr: Ref<usize>, somevalue: usize) -> AmxResult<f32> {
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

        let refer = amx.get_ref::<f32>(10)?;
        Ok(*refer)
    }

    unsafe fn raw_callback(amx: *mut AMX, args: *mut i32) -> i32 {
        let amx = Amx::new(amx, 0);
        let ptr = Ref::<usize>::from_raw(&amx, args.offset(1).read()).unwrap();
        let somevalue = usize::from_raw(&amx, args.offset(2).read()).unwrap();
        let rr = UnsizedBuffer::from_raw(&amx, args.offset(3).read()).unwrap();

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