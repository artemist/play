#[doc = "Register `SYSAHBCLKCTRL` reader"]
pub type R = crate::R<SysahbclkctrlSpec>;
#[doc = "Register `SYSAHBCLKCTRL` writer"]
pub type W = crate::W<SysahbclkctrlSpec>;
#[doc = "Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sys {
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Sys> for bool {
    #[inline(always)]
    fn from(variant: Sys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS` reader - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
pub type SysR = crate::BitReader<Sys>;
impl SysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sys {
        match self.bits {
            true => Sys::Enable,
            _ => unreachable!(),
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sys::Enable
    }
}
#[doc = "Field `SYS` writer - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
pub type SysW<'a, REG> = crate::BitWriter<'a, REG, Sys>;
impl<'a, REG> SysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sys::Enable)
    }
}
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rom {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Rom> for bool {
    #[inline(always)]
    fn from(variant: Rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Enables clock for ROM."]
pub type RomR = crate::BitReader<Rom>;
impl RomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rom {
        match self.bits {
            false => Rom::Disable,
            true => Rom::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rom::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rom::Enable
    }
}
#[doc = "Field `ROM` writer - Enables clock for ROM."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG, Rom>;
impl<'a, REG> RomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Enable)
    }
}
#[doc = "Enables clock for RAM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ram> for bool {
    #[inline(always)]
    fn from(variant: Ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM` reader - Enables clock for RAM."]
pub type RamR = crate::BitReader<Ram>;
impl RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram {
        match self.bits {
            false => Ram::Disable,
            true => Ram::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ram::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ram::Enable
    }
}
#[doc = "Field `RAM` writer - Enables clock for RAM."]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG, Ram>;
impl<'a, REG> RamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram::Enable)
    }
}
#[doc = "Enables clock for flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashreg {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Flashreg> for bool {
    #[inline(always)]
    fn from(variant: Flashreg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHREG` reader - Enables clock for flash register interface."]
pub type FlashregR = crate::BitReader<Flashreg>;
impl FlashregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashreg {
        match self.bits {
            false => Flashreg::Disabled,
            true => Flashreg::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flashreg::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flashreg::Enabled
    }
}
#[doc = "Field `FLASHREG` writer - Enables clock for flash register interface."]
pub type FlashregW<'a, REG> = crate::BitWriter<'a, REG, Flashreg>;
impl<'a, REG> FlashregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flashreg::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flashreg::Enabled)
    }
}
#[doc = "Enables clock for flash array access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flasharray {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Flasharray> for bool {
    #[inline(always)]
    fn from(variant: Flasharray) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHARRAY` reader - Enables clock for flash array access."]
pub type FlasharrayR = crate::BitReader<Flasharray>;
impl FlasharrayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flasharray {
        match self.bits {
            false => Flasharray::Disabled,
            true => Flasharray::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flasharray::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flasharray::Enabled
    }
}
#[doc = "Field `FLASHARRAY` writer - Enables clock for flash array access."]
pub type FlasharrayW<'a, REG> = crate::BitWriter<'a, REG, Flasharray>;
impl<'a, REG> FlasharrayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flasharray::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flasharray::Enabled)
    }
}
#[doc = "Enables clock for I2C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<I2c> for bool {
    #[inline(always)]
    fn from(variant: I2c) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` reader - Enables clock for I2C."]
pub type I2cR = crate::BitReader<I2c>;
impl I2cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c {
        match self.bits {
            false => I2c::Disable,
            true => I2c::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2c::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2c::Enable
    }
}
#[doc = "Field `I2C` writer - Enables clock for I2C."]
pub type I2cW<'a, REG> = crate::BitWriter<'a, REG, I2c>;
impl<'a, REG> I2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::Enable)
    }
}
#[doc = "Enables clock for GPIO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Gpio> for bool {
    #[inline(always)]
    fn from(variant: Gpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO."]
pub type GpioR = crate::BitReader<Gpio>;
impl GpioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio {
        match self.bits {
            false => Gpio::Disable,
            true => Gpio::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio::Enable
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO."]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG, Gpio>;
impl<'a, REG> GpioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::Enable)
    }
}
#[doc = "Enables clock for 16-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct16b0 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ct16b0> for bool {
    #[inline(always)]
    fn from(variant: Ct16b0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B0` reader - Enables clock for 16-bit counter/timer 0."]
pub type Ct16b0R = crate::BitReader<Ct16b0>;
impl Ct16b0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct16b0 {
        match self.bits {
            false => Ct16b0::Disable,
            true => Ct16b0::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ct16b0::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ct16b0::Enable
    }
}
#[doc = "Field `CT16B0` writer - Enables clock for 16-bit counter/timer 0."]
pub type Ct16b0W<'a, REG> = crate::BitWriter<'a, REG, Ct16b0>;
impl<'a, REG> Ct16b0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b0::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b0::Enable)
    }
}
#[doc = "Enables clock for 16-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct16b1 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ct16b1> for bool {
    #[inline(always)]
    fn from(variant: Ct16b1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B1` reader - Enables clock for 16-bit counter/timer 1."]
pub type Ct16b1R = crate::BitReader<Ct16b1>;
impl Ct16b1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct16b1 {
        match self.bits {
            false => Ct16b1::Disable,
            true => Ct16b1::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ct16b1::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ct16b1::Enable
    }
}
#[doc = "Field `CT16B1` writer - Enables clock for 16-bit counter/timer 1."]
pub type Ct16b1W<'a, REG> = crate::BitWriter<'a, REG, Ct16b1>;
impl<'a, REG> Ct16b1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b1::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b1::Enable)
    }
}
#[doc = "Enables clock for 32-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32b0 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ct32b0> for bool {
    #[inline(always)]
    fn from(variant: Ct32b0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32B0` reader - Enables clock for 32-bit counter/timer 0."]
pub type Ct32b0R = crate::BitReader<Ct32b0>;
impl Ct32b0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32b0 {
        match self.bits {
            false => Ct32b0::Disable,
            true => Ct32b0::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ct32b0::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ct32b0::Enable
    }
}
#[doc = "Field `CT32B0` writer - Enables clock for 32-bit counter/timer 0."]
pub type Ct32b0W<'a, REG> = crate::BitWriter<'a, REG, Ct32b0>;
impl<'a, REG> Ct32b0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b0::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b0::Enable)
    }
}
#[doc = "Enables clock for 32-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32b1 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ct32b1> for bool {
    #[inline(always)]
    fn from(variant: Ct32b1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32B1` reader - Enables clock for 32-bit counter/timer 1."]
pub type Ct32b1R = crate::BitReader<Ct32b1>;
impl Ct32b1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32b1 {
        match self.bits {
            false => Ct32b1::Disable,
            true => Ct32b1::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ct32b1::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ct32b1::Enable
    }
}
#[doc = "Field `CT32B1` writer - Enables clock for 32-bit counter/timer 1."]
pub type Ct32b1W<'a, REG> = crate::BitWriter<'a, REG, Ct32b1>;
impl<'a, REG> Ct32b1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b1::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b1::Enable)
    }
}
#[doc = "Enables clock for SPI0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp0 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ssp0> for bool {
    #[inline(always)]
    fn from(variant: Ssp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP0` reader - Enables clock for SPI0."]
pub type Ssp0R = crate::BitReader<Ssp0>;
impl Ssp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp0 {
        match self.bits {
            false => Ssp0::Disable,
            true => Ssp0::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ssp0::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ssp0::Enable
    }
}
#[doc = "Field `SSP0` writer - Enables clock for SPI0."]
pub type Ssp0W<'a, REG> = crate::BitWriter<'a, REG, Ssp0>;
impl<'a, REG> Ssp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp0::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp0::Enable)
    }
}
#[doc = "Enables clock for UART. See Section 3.1 for part specific details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Uart> for bool {
    #[inline(always)]
    fn from(variant: Uart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART` reader - Enables clock for UART. See Section 3.1 for part specific details."]
pub type UartR = crate::BitReader<Uart>;
impl UartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart {
        match self.bits {
            false => Uart::Disable,
            true => Uart::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uart::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uart::Enable
    }
}
#[doc = "Field `UART` writer - Enables clock for UART. See Section 3.1 for part specific details."]
pub type UartW<'a, REG> = crate::BitWriter<'a, REG, Uart>;
impl<'a, REG> UartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart::Enable)
    }
}
#[doc = "Enables clock for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Adc> for bool {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Enables clock for ADC."]
pub type AdcR = crate::BitReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            false => Adc::Disable,
            true => Adc::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adc::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adc::Enable
    }
}
#[doc = "Field `ADC` writer - Enables clock for ADC."]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG, Adc>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Enable)
    }
}
#[doc = "Enables clock for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Enables clock for WDT."]
pub type WdtR = crate::BitReader<Wdt>;
impl WdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt {
        match self.bits {
            false => Wdt::Disable,
            true => Wdt::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wdt::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wdt::Enable
    }
}
#[doc = "Field `WDT` writer - Enables clock for WDT."]
pub type WdtW<'a, REG> = crate::BitWriter<'a, REG, Wdt>;
impl<'a, REG> WdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Enable)
    }
}
#[doc = "Enables clock for I/O configuration block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocon {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Iocon> for bool {
    #[inline(always)]
    fn from(variant: Iocon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables clock for I/O configuration block."]
pub type IoconR = crate::BitReader<Iocon>;
impl IoconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocon {
        match self.bits {
            false => Iocon::Disable,
            true => Iocon::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Iocon::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Iocon::Enable
    }
}
#[doc = "Field `IOCON` writer - Enables clock for I/O configuration block."]
pub type IoconW<'a, REG> = crate::BitWriter<'a, REG, Iocon>;
impl<'a, REG> IoconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Iocon::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Iocon::Enable)
    }
}
#[doc = "Enables clock for C_CAN. See Section 3.1 for part specific details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Can {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Can> for bool {
    #[inline(always)]
    fn from(variant: Can) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN` reader - Enables clock for C_CAN. See Section 3.1 for part specific details."]
pub type CanR = crate::BitReader<Can>;
impl CanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Can {
        match self.bits {
            false => Can::Disable,
            true => Can::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Can::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Can::Enable
    }
}
#[doc = "Field `CAN` writer - Enables clock for C_CAN. See Section 3.1 for part specific details."]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG, Can>;
impl<'a, REG> CanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Can::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Can::Enable)
    }
}
#[doc = "Enables clock for SPI1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp1 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ssp1> for bool {
    #[inline(always)]
    fn from(variant: Ssp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP1` reader - Enables clock for SPI1."]
pub type Ssp1R = crate::BitReader<Ssp1>;
impl Ssp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp1 {
        match self.bits {
            false => Ssp1::Disable,
            true => Ssp1::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ssp1::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ssp1::Enable
    }
}
#[doc = "Field `SSP1` writer - Enables clock for SPI1."]
pub type Ssp1W<'a, REG> = crate::BitWriter<'a, REG, Ssp1>;
impl<'a, REG> Ssp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
    #[inline(always)]
    pub fn sys(&self) -> SysR {
        SysR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FlashregR {
        FlashregR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FlasharrayR {
        FlasharrayR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO."]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> Ct16b0R {
        Ct16b0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> Ct16b1R {
        Ct16b1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> Ct32b0R {
        Ct32b0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> Ct32b1R {
        Ct32b1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    pub fn ssp0(&self) -> Ssp0R {
        Ssp0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables clock for UART. See Section 3.1 for part specific details."]
    #[inline(always)]
    pub fn uart(&self) -> UartR {
        UartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables clock for WDT."]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IoconR {
        IoconR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables clock for C_CAN. See Section 3.1 for part specific details."]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables clock for SPI1."]
    #[inline(always)]
    pub fn ssp1(&self) -> Ssp1R {
        Ssp1R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SysW<SysahbclkctrlSpec> {
        SysW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<SysahbclkctrlSpec> {
        RomW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RamW<SysahbclkctrlSpec> {
        RamW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    #[must_use]
    pub fn flashreg(&mut self) -> FlashregW<SysahbclkctrlSpec> {
        FlashregW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    #[must_use]
    pub fn flasharray(&mut self) -> FlasharrayW<SysahbclkctrlSpec> {
        FlasharrayW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<SysahbclkctrlSpec> {
        I2cW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables clock for GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<SysahbclkctrlSpec> {
        GpioW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn ct16b0(&mut self) -> Ct16b0W<SysahbclkctrlSpec> {
        Ct16b0W::new(self, 7)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn ct16b1(&mut self) -> Ct16b1W<SysahbclkctrlSpec> {
        Ct16b1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn ct32b0(&mut self) -> Ct32b0W<SysahbclkctrlSpec> {
        Ct32b0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn ct32b1(&mut self) -> Ct32b1W<SysahbclkctrlSpec> {
        Ct32b1W::new(self, 10)
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    #[must_use]
    pub fn ssp0(&mut self) -> Ssp0W<SysahbclkctrlSpec> {
        Ssp0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enables clock for UART. See Section 3.1 for part specific details."]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UartW<SysahbclkctrlSpec> {
        UartW::new(self, 12)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> AdcW<SysahbclkctrlSpec> {
        AdcW::new(self, 13)
    }
    #[doc = "Bit 15 - Enables clock for WDT."]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WdtW<SysahbclkctrlSpec> {
        WdtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    #[must_use]
    pub fn iocon(&mut self) -> IoconW<SysahbclkctrlSpec> {
        IoconW::new(self, 16)
    }
    #[doc = "Bit 17 - Enables clock for C_CAN. See Section 3.1 for part specific details."]
    #[inline(always)]
    #[must_use]
    pub fn can(&mut self) -> CanW<SysahbclkctrlSpec> {
        CanW::new(self, 17)
    }
    #[doc = "Bit 18 - Enables clock for SPI1."]
    #[inline(always)]
    #[must_use]
    pub fn ssp1(&mut self) -> Ssp1W<SysahbclkctrlSpec> {
        Ssp1W::new(self, 18)
    }
}
#[doc = "System AHB clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysahbclkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysahbclkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysahbclkctrlSpec;
impl crate::RegisterSpec for SysahbclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysahbclkctrl::R`](R) reader structure"]
impl crate::Readable for SysahbclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysahbclkctrl::W`](W) writer structure"]
impl crate::Writable for SysahbclkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0x085f"]
impl crate::Resettable for SysahbclkctrlSpec {
    const RESET_VALUE: u32 = 0x085f;
}
