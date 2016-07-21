use common::volatile_cell::VolatileCell;

pub struct Control(VolatileCell<u32>);

impl Control {
    pub fn set(&self, val: u32) {
        self.0.set(val)
    }

    pub fn get(&self) -> u32 {
        self.0.get()
    }

    pub fn set_bit(&self, bit: usize) {
        let old = self.0.get();
        self.0.set(old | (1 << bit))
    }

    pub fn clear_bit(&self, bit: usize) {
        let old = self.0.get();
        self.0.set(old & !(1 << bit))
    }
}

pub struct Pin {
    pub select: VolatileCell<Function>,
    pub control: Control
}

pub struct Registers {
    pub diom0: Pin, pub diom1: Pin, pub diom2: Pin,
    pub diom3: Pin, pub diom4: Pin,

    pub dioa0: Pin, pub dioa1: Pin, pub dioa2: Pin,
    pub dioa3: Pin, pub dioa4: Pin, pub dioa5: Pin,
    pub dioa6: Pin, pub dioa7: Pin, pub dioa8: Pin,
    pub dioa9: Pin, pub dioa10: Pin, pub dioa11: Pin,
    pub dioa12: Pin, pub dioa13: Pin, pub dioa14: Pin,

    pub diob0: Pin, pub diob1: Pin, pub diob2: Pin,
    pub diob3: Pin, pub diob4: Pin, pub diob5: Pin,
    pub diob6: Pin, pub diob7: Pin
}

pub const PINMUX : *mut Registers = 0x40060000 as *mut Registers;

#[repr(u32)]
pub enum Function {
    Default = 0,
    Gpio0Gpio0 = 1,
    Gpio0Gpio1 = 2,
    Gpio0Gpio2 = 3,
    Gpio0Gpio3 = 4,
    Gpio0Gpio4 = 5,
    Gpio0Gpio5 = 6,
    Gpio0Gpio6 = 7,
    Gpio0Gpio7 = 8,
    Gpio0Gpio8 = 9,
    Gpio0Gpio9 = 10,
    Gpio0Gpio10 = 11,
    Gpio0Gpio11 = 12,
    Gpio0Gpio12 = 13,
    Gpio0Gpio13 = 14,
    Gpio0Gpio14 = 15,
    Gpio0Gpio15 = 16,
    Gpio1Gpio0 = 17,
    Gpio1Gpio1 = 18,
    Gpio1Gpio2	= 19,
    Gpio1Gpio3	= 20,
    Gpio1Gpio4	= 21,
    Gpio1Gpio5	= 22,
    Gpio1Gpio6	= 23,
    Gpio1Gpio7	= 24,
    Gpio1Gpio8 = 25,
    Gpio1Gpio9 = 26,
    Gpio1Gpio10 = 27,
    Gpio1Gpio11 = 28,
    Gpio1Gpio12 = 29,
    Gpio1Gpio13 = 30,
    Gpio1Gpio14 = 31,
    Gpio1Gpio15 = 32,
    I2C0Scl = 33,
    I2C0Sda = 34,
    I2C1Scl = 35,
    I2C1SDA = 36,
    I2cs0Scl = 37,
    I2cs0Sda = 38,
    PmuBrownoutDet = 39,
    PmuTestbus0 = 40,
    PmuTestbus1 = 41,
    PmuTestbus2 = 42,
    PmuTestbus3 = 43,
    PmuTestbus4 = 44,
    PmuTestbus5 = 45,
    PmuTestbus6 = 46,
    PmuTestbus7 = 47,
    Rtc0RtcClkTest = 48,
    Spi1Spiclk	= 49,
    Spi1Spicsb	= 50,
    Spi1Spimiso = 51,
    Spi1Spimosi = 52,
    Sps0Testbus0 = 53,
    Sps0Testbus1 = 54,
    Sps0Testbus2 = 55,
    Sps0Testbus3 = 56,
    Sps0Testbus4 = 57,
    Sps0Testbus5 = 58,
    Sps0Testbus6 = 59,
    Sps0Testbus7 = 60,
    Temp0TstAdcClk = 61,
    Temp0TstAdcHiSer = 62,
    Temp0TstAdcLoSer = 63,
    Temp0TstAdcVldSer = 64,
    Trng0TrngRoDiv = 65,
    Trng0TrngRoRefDiv = 66,
    Uart0Cts = 67,
    Uart0Rts = 68,
    Uart0Rx = 69,
    Uart0Tx = 70,
    Uart1Cts = 71,
    Uart1Rts = 72,
    Uart1Rx = 73,
    Uart1Tx = 74,
    Uart2Cts = 75,
    Uart2Rts = 76,
    Uart2Rx = 77,
    Uart2Tx = 78,
    Usb0ExtDmPullupEn = 79,
    Usb0ExtDpRpu1Enb = 80,
    Usb0ExtDpRpu2Enb = 81,
    Usb0ExtFsEdgeSel = 82,
    Usb0ExtRxDmi = 83,
    Usb0ExtRxDpi = 84,
    Usb0ExtRxRcv = 85,
    Usb0ExtSuspendb = 86,
    Usb0ExtTxDmo	= 87,
    Usb0ExtTxDpo	= 88,
    Usb0ExtTxOeb	= 89,
    Volt0TstNegGlitchDet = 90,
    Volt0TstPosGlitchDet = 91,
    Xo0Testbus0 = 92,
    Xo0Testbus1 = 93,
    Xo0Testbus2 = 94,
    Xo0Testbus3 = 95,
    Xo0Testbus4 = 96,
    Xo0Testbus5 = 97,
    Xo0testbus6 = 98,
    Xo0Testbus7 = 99
}

