//! USB Interface descriptor.


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct InterfaceDescriptor {
	bLength: u8,
	bDescriptorType: u8,
	bInterfaceNumber: u8,
	bAlternateSetting: u8,

	bNumEndpoints: u8,
	bInterfaceClass: u8,
	bInterfaceSubClass: u8,
	bInterfaceProtocol: u8,

	iInterface: u8,
}
