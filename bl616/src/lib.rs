#![doc = "Peripheral access API for BL616 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 0] = [];
#[doc = "Global configuration register"]
pub struct GLB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GLB {}
impl GLB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const glb::RegisterBlock = 0x2000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const glb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GLB {
    type Target = glb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GLB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLB").finish()
    }
}
#[doc = "Global configuration register"]
pub mod glb;
#[doc = "Generic DAC, ADC and ACOMP interface control"]
pub struct GPIP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIP {}
impl GPIP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpip::RegisterBlock = 0x2000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpip::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIP {
    type Target = gpip::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIP").finish()
    }
}
#[doc = "Generic DAC, ADC and ACOMP interface control"]
pub mod gpip;
#[doc = "Automatic Gain Control"]
pub struct AGC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AGC {}
impl AGC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const agc::RegisterBlock = 0x2000_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const agc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AGC {
    type Target = agc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AGC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC").finish()
    }
}
#[doc = "Automatic Gain Control"]
pub mod agc;
#[doc = "Secure Engine"]
pub struct SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC {}
impl SEC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sec::RegisterBlock = 0x2000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SEC {
    type Target = sec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC").finish()
    }
}
#[doc = "Secure Engine"]
pub mod sec;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x2000_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x2000_a100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x2000_a200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Inter-Integrated Circuit bus"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x2000_a300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "Inter-Integrated Circuit bus"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x2000_a900 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
#[doc = "Inter-Integrated Circuit bus"]
pub mod i2c;
#[doc = "Pulse-Width Modulation module"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm::RegisterBlock = 0x2000_a400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM").finish()
    }
}
#[doc = "Pulse-Width Modulation module"]
pub mod pwm;
#[doc = "Timer control"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer::RegisterBlock = 0x2000_a500 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER").finish()
    }
}
#[doc = "Timer control"]
pub mod timer;
#[doc = "Infrared receiver module"]
pub struct IR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IR {}
impl IR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ir::RegisterBlock = 0x2000_a600 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ir::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IR {
    type Target = ir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR").finish()
    }
}
#[doc = "Infrared receiver module"]
pub mod ir;
#[doc = "Checksum peripheral"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x2000_a700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "Checksum peripheral"]
pub mod crc;
#[doc = "MIPI Display Bus Interface"]
pub struct DBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBI {}
impl DBI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dbi::RegisterBlock = 0x2000_a800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DBI {
    type Target = dbi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DBI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI").finish()
    }
}
#[doc = "MIPI Display Bus Interface"]
pub mod dbi;
#[doc = "ISO 11898 communication protocol"]
pub struct ISO11898 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ISO11898 {}
impl ISO11898 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const iso11898::RegisterBlock = 0x2000_aa00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iso11898::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ISO11898 {
    type Target = iso11898::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ISO11898 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISO11898").finish()
    }
}
#[doc = "ISO 11898 communication protocol"]
pub mod iso11898;
#[doc = "Inter-IC Sound controller"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2s::RegisterBlock = 0x2000_ab00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S").finish()
    }
}
#[doc = "Inter-IC Sound controller"]
pub mod i2s;
#[doc = "Audio Analog-to-Digital Converter"]
pub struct AADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AADC {}
impl AADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aadc::RegisterBlock = 0x2000_ac00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aadc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AADC {
    type Target = aadc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AADC").finish()
    }
}
#[doc = "Audio Analog-to-Digital Converter"]
pub mod aadc;
#[doc = "Quad Serial Flash control"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x2000_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
#[doc = "Quad Serial Flash control"]
pub mod flash;
#[doc = "Power-Down Sleep control"]
pub struct PDS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDS {}
impl PDS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pds::RegisterBlock = 0x2000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pds::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PDS {
    type Target = pds::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDS").finish()
    }
}
#[doc = "Power-Down Sleep control"]
pub mod pds;
#[doc = "Hibernate (Deep sleep) control"]
pub struct HBN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HBN {}
impl HBN {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hbn::RegisterBlock = 0x2000_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hbn::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HBN {
    type Target = hbn::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HBN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HBN").finish()
    }
}
#[doc = "Hibernate (Deep sleep) control"]
pub mod hbn;
#[doc = "Pseudo Static Random Access Memory control"]
pub struct PSRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PSRAM {}
impl PSRAM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const psram::RegisterBlock = 0x2005_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const psram::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PSRAM {
    type Target = psram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PSRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM").finish()
    }
}
#[doc = "Pseudo Static Random Access Memory control"]
pub mod psram;
#[doc = "Audio Pulse-Width Modulation controller"]
pub struct APWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APWM {}
impl APWM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apwm::RegisterBlock = 0x2005_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apwm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APWM {
    type Target = apwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APWM").finish()
    }
}
#[doc = "Audio Pulse-Width Modulation controller"]
pub mod apwm;
#[doc = "eFuse memory control"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x2005_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "eFuse memory control"]
pub mod efuse;
#[doc = "Digital Video Port control"]
pub struct DVP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DVP0 {}
impl DVP0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dvp::RegisterBlock = 0x2005_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dvp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DVP0 {
    type Target = dvp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DVP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVP0").finish()
    }
}
#[doc = "Digital Video Port control"]
pub mod dvp;
#[doc = "Motion JPEG encoder"]
pub struct MJPEG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MJPEG {}
impl MJPEG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mjpeg::RegisterBlock = 0x2005_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mjpeg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MJPEG {
    type Target = mjpeg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MJPEG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MJPEG").finish()
    }
}
#[doc = "Motion JPEG encoder"]
pub mod mjpeg;
#[doc = "Secure Digital host control"]
pub struct SDH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDH {}
impl SDH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sdh::RegisterBlock = 0x2006_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdh::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SDH {
    type Target = sdh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SDH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDH").finish()
    }
}
#[doc = "Secure Digital host control"]
pub mod sdh;
#[doc = "Ethernet Media Access Control"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const emac::RegisterBlock = 0x2007_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EMAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMAC").finish()
    }
}
#[doc = "Ethernet Media Access Control"]
pub mod emac;
#[doc = "Universal Serial Bus host"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb::RegisterBlock = 0x2007_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB").finish()
    }
}
#[doc = "Universal Serial Bus host"]
pub mod usb;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GLB"]
    pub GLB: GLB,
    #[doc = "GPIP"]
    pub GPIP: GPIP,
    #[doc = "AGC"]
    pub AGC: AGC,
    #[doc = "SEC"]
    pub SEC: SEC,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "IR"]
    pub IR: IR,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DBI"]
    pub DBI: DBI,
    #[doc = "ISO11898"]
    pub ISO11898: ISO11898,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "AADC"]
    pub AADC: AADC,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "PDS"]
    pub PDS: PDS,
    #[doc = "HBN"]
    pub HBN: HBN,
    #[doc = "PSRAM"]
    pub PSRAM: PSRAM,
    #[doc = "APWM"]
    pub APWM: APWM,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "DVP0"]
    pub DVP0: DVP0,
    #[doc = "MJPEG"]
    pub MJPEG: MJPEG,
    #[doc = "SDH"]
    pub SDH: SDH,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "USB"]
    pub USB: USB,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GLB: GLB {
                _marker: PhantomData,
            },
            GPIP: GPIP {
                _marker: PhantomData,
            },
            AGC: AGC {
                _marker: PhantomData,
            },
            SEC: SEC {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            IR: IR {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            DBI: DBI {
                _marker: PhantomData,
            },
            ISO11898: ISO11898 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            AADC: AADC {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            PDS: PDS {
                _marker: PhantomData,
            },
            HBN: HBN {
                _marker: PhantomData,
            },
            PSRAM: PSRAM {
                _marker: PhantomData,
            },
            APWM: APWM {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            DVP0: DVP0 {
                _marker: PhantomData,
            },
            MJPEG: MJPEG {
                _marker: PhantomData,
            },
            SDH: SDH {
                _marker: PhantomData,
            },
            EMAC: EMAC {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
        }
    }
}
