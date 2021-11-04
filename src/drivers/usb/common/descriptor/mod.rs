//! USB specification descriptors.


mod bos;
mod config;
mod device;
mod endpoint;
mod interface;
mod string;



pub use self::bos::BOSDescriptor;
pub use self::config::ConfigurationDescriptor;
pub use self::device::DeviceDescriptor;
pub use self::endpoint::EndpointDescriptor;
pub use self::interface::InterfaceDescriptor;
pub use self::string::{ StringDescriptor, /*StringDescriptorZero*/ };
