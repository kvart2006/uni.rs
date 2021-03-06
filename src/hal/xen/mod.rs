//! Xen backend for Uni.rs
//!
//! Note: This backend is enabled by the feature named *xen* and thus might
//! not be available depending on your build's configuration.

use io::Write;

use alloc_uni;

use hal;

use hal::arch::utils::wmb;
use hal::xen::store::XenStore;

use thread::Scheduler;

// In order to use stdout, heap must be initialized, so "raw" console is used
// before that
macro_rules! raw_println {
    ($fmt:expr) => {
        raw_print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        raw_print!(concat!($fmt, "\r\n"), $($arg)*)
    }
}

macro_rules! raw_print {
    ($($arg:tt)*) => {{
        use $crate::io::Write;

        $crate::hal::console().write_fmt(format_args!($($arg)*)).unwrap();
    }}
}

mod hypercall;

pub mod arch;
pub mod defs;
pub mod boot;
pub mod ring;
pub mod grant;
pub mod store;
pub mod event;
pub mod sched;
pub mod memory;
pub mod console;
#[cfg(feature = "net")] pub mod net;

extern "C" {
    // This symbol must be present in code using libxen
    pub static mut shared_info: self::defs::SharedInfo;
}

pub fn enable_upcalls() -> u8 {
    unsafe {
        let ret = shared_info.vcpu_info[0].evtchn_upcall_mask;

        wmb();
        shared_info.vcpu_info[0].evtchn_upcall_mask = 0;
        wmb();

        ret
    }
}

pub fn disable_upcalls() -> u8 {
    unsafe {
        let ret = shared_info.vcpu_info[0].evtchn_upcall_mask;

        shared_info.vcpu_info[0].evtchn_upcall_mask = 1;
        wmb();

        ret
    }
}

pub fn set_upcalls_state(state: u8) {
    unsafe {
        wmb();
        shared_info.vcpu_info[0].evtchn_upcall_mask = state;
        wmb();
    }
}

extern {
    fn main(_: isize, _: *const *const u8) -> isize;
}

#[inline]
#[cfg(feature = "net")]
fn net_init() {
    ::net::Stack::init();
}

#[inline]
#[cfg(not(feature = "net"))]
fn net_init() {
    println!("No network configured. Skipping...");
}

#[no_mangle]
/// Entry point of the application called by boot assembly
pub extern "C" fn uni_rust_entry() -> ! {
    alloc_uni::init();

    boot::init();

    raw_println!("Uni.rs is booting");

    boot::init_memory();

    grant::Table::init().expect("Fail to initialize Xen's Grant Table");

    event::init();

    unsafe {
        console::console().init_input();
        XenStore::init_event();
    }

    hal::local_irq_enable();

    // Spawn main thread
    Scheduler::spawn(|| {
        println!("Main thread started");

        net_init();

        println!("Uni.rs is now ready");
        println!("Control is now transfered to the application");
        println!("");

        let app_ret = unsafe {
            main(0, ::core::ptr::null())
        };

        hal::local_irq_disable();

        hal::console().flush().unwrap();

        hal::app::exit(app_ret);

        panic!("Failed to poweroff the machine !");
    });

    Scheduler::schedule();

    unreachable!();
}
