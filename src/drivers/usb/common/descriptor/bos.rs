//! USB Binary Device Object descriptor.




#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct BOSDescriptor {
	bLength: u8,
	bDescriptorType: u8,
	wTotalLength: u16,

	bNumDeviceCaps: u8,
}
