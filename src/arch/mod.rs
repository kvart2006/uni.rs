#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_common;

#[cfg(target_arch = "x86_64")]
#[path="x86_64"]
mod imp {
    pub mod compiler {}
    pub mod defs;
    pub mod xen;
    pub use ::arch::x86_common::barrier;
    pub use ::arch::x86_common::init;
}

#[cfg(target_arch = "x86")]
#[path="x86"]
mod imp {
    pub mod compiler {}
    pub mod defs;
    pub mod xen;
    pub use ::arch::x86_common::barrier;
    pub use ::arch::x86_common::init;
}

// Exported functions
pub use self::imp::init;

// Exported modules
pub use self::imp::defs;
pub use self::imp::xen;
pub use self::imp::barrier;
pub use self::imp::compiler;
