use boxed::Box;
use string::String;

use sync::Arc;

use sync::spin::RwLock;

use net::defs::{HwAddr, Ipv4Addr, Device};

use hal::net::HwInterface;

// XXX: Should this be in net::defs ?
/// IPv4 configuration of an interface
pub struct V4Configuration {
    /// Main IPv4 address
    pub ipv4: Ipv4Addr,
    /// Subnet mask
    pub ipv4_mask: Ipv4Addr,
    /// Gateway IPv4 address
    pub ipv4_gateway: Ipv4Addr,
}

/// A network interface
pub struct Interface {
    name: String,
    hw_addr: HwAddr,
    conf: V4Configuration,
    pv_device: Option<Box<HwInterface>>,
}

unsafe impl Send for Interface {}

impl Interface {
    /// Creates a new network interface
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Interface {
            name: String::new(),
            hw_addr: HwAddr::empty(),
            conf: V4Configuration {
                ipv4: Ipv4Addr::new(0, 0, 0, 0),
                ipv4_mask: Ipv4Addr::new(0, 0, 0, 0),
                ipv4_gateway: Ipv4Addr::new(0, 0, 0, 0),
            },
            pv_device: None,
        }))
    }

    #[inline]
    /// Returns a reference over the name of the interface
    pub fn name_ref(&self) -> &str {
        &self.name
    }

    #[inline]
    /// Returns a reference over the hardware of the interface
    pub fn hw_addr_ref(&self) -> &HwAddr {
        &self.hw_addr
    }

    #[inline]
    /// Returns a reference over the IPv4 configuration of the interface
    pub fn v4_configuration_ref(&self) -> &V4Configuration {
        &self.conf
    }

    #[inline]
    /// Returns a mutable reference over the name of the interface
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    #[inline]
    /// Returns a mutable reference over the hardware of the interface
    pub fn hw_addr_mut(&mut self) -> &mut HwAddr {
        &mut self.hw_addr
    }

    #[inline]
    /// Returns a mutable reference over the IPv4 configuration of the interface
    pub fn v4_configuration_mut(&mut self) -> &mut V4Configuration {
        &mut self.conf
    }

    #[inline]
    #[doc(hidden)]
    pub fn pv_device_set(&mut self, pv: Box<HwInterface>) {
        self.pv_device = Some(pv);
    }

    #[inline]
    /// Refresh underlying driver
    pub fn refresh(&mut self) {
        self.pv_device.as_mut().unwrap().refresh();
    }
}