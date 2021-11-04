//! USB String descriptor.


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct StringDescriptor<const SIZE: usize> {
	/// Size in bytes of this descriptor.
	bLength: u8,

	/// String Descriptor.
	bDescriptorType: u8,

	/// Actual string.
	bString: [u8; SIZE],
}