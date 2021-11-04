//! USB Device descriptor.



#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct DeviceDescriptor {
    /// Size of this descriptor in bytes.
    bLength:  u8,
    /// DEVICE Descriptor Type.
    bDescriptorType:  u8,
    /// BUSB Specification Release Number in Binary-Coded Decimal (i.e., 2.10 is 210H). This field identifies the release of the USB Specification with which the device and its descriptors are compliant.
    bcdUSB: u16,

    /// Class code (assigned by the USB-IF). \li If this field is reset to zero, each interface within a configuration specifies its own class information and the various interfaces operate independently. \li If this field is set to a value between 1 and FEH, the device supports different class specifications on different interfaces and the interfaces may not operate independently. This value identifies the class definition used for the aggregate interfaces. \li If this field is set to FFH, the device class is vendor-specific.
    bDeviceClass:  u8,
    /// Subclass code (assigned by the USB-IF). These codes are qualified by the value of the bDeviceClass field. \li If the bDeviceClass field is reset to zero, this field must also be reset to zero. \li If the bDeviceClass field is not set to FFH, all values are reserved for assignment by the USB-IF.
    bDeviceSubClass:  u8,
    /// Protocol code (assigned by the USB-IF). These codes are qualified by the value of the bDeviceClass and the bDeviceSubClass fields. If a device supports class-specific protocols on a device basis as opposed to an interface basis, this code identifies the protocols that the device uses as defined by the specification of the device class. \li If this field is reset to zero, the device does not use class-specific protocols on a device basis. However, it may use classspecific protocols on an interface basis. \li If this field is set to FFH, the device uses a vendor-specific protocol on a device basis.
    bDeviceProtocol:  u8,
    /// Maximum packet size for endpoint zero (only 8, 16, 32, or 64 are valid). For HS devices is fixed to 64.
    bMaxPacketSize0:  u8,

    /// Vendor ID (assigned by the USB-IF).
    idVendor: u16,
    /// Product ID (assigned by the manufacturer).
    idProduct: u16,

    /// Device release number in binary-coded decimal.
    bcdDevice: u16,
    /// Index of string descriptor describing manufacturer.
    iManufacturer: u8,
    /// Index of string descriptor describing product.
    iProduct: u8,

    /// Index of string descriptor describing the device's serial number.
    iSerialNumber: u8,
    /// Number of possible configurations.
    bNumConfigurations: u8,
}



// Compile time assertion.
const __BAD_DEVICE_DESCRIPTOR_SIZE__: [(); 0] = [(); if core::mem::size_of::<DeviceDescriptor>() == 18 { 0 } else { 1 }];
