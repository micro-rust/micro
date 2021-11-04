//! USB Configuration descriptor.


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct ConfigurationDescriptor {
	bLength: u8,
	bDescriptorType: u8,
	wTotalLength: u16,

	bNumInterfaces: u8,
	bConfigurationValue: u8,
	iConfiguration: u8,
	bmAttributes: u8,
	bMaxPower: u8,
}
