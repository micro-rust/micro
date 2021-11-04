//! USB Endpoint decriptor.


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct EndpointDescriptor {
	bLength: u8,
	bDescriptorType: u8,
	bEndpointAddress: u8,
	bmAttributes: u8,

	wMaxPacketSize: u16,
	bInterval: u8,
}
