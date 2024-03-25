#[doc = "Register `PRESETCTRL` reader"]
pub type R = crate::R<PresetctrlSpec>;
#[doc = "Register `PRESETCTRL` writer"]
pub type W = crate::W<PresetctrlSpec>;
#[doc = "SPI0 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp0RstN {
    #[doc = "0: Resets the SPI0 peripheral."]
    Spio0reset = 0,
    #[doc = "1: SPI0 reset de-asserted."]
    Spio0noreset = 1,
}
impl From<Ssp0RstN> for bool {
    #[inline(always)]
    fn from(variant: Ssp0RstN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP0_RST_N` reader - SPI0 reset control"]
pub type Ssp0RstNR = crate::BitReader<Ssp0RstN>;
impl Ssp0RstNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp0RstN {
        match self.bits {
            false => Ssp0RstN::Spio0reset,
            true => Ssp0RstN::Spio0noreset,
        }
    }
    #[doc = "Resets the SPI0 peripheral."]
    #[inline(always)]
    pub fn is_spio0reset(&self) -> bool {
        *self == Ssp0RstN::Spio0reset
    }
    #[doc = "SPI0 reset de-asserted."]
    #[inline(always)]
    pub fn is_spio0noreset(&self) -> bool {
        *self == Ssp0RstN::Spio0noreset
    }
}
#[doc = "Field `SSP0_RST_N` writer - SPI0 reset control"]
pub type Ssp0RstNW<'a, REG> = crate::BitWriter<'a, REG, Ssp0RstN>;
impl<'a, REG> Ssp0RstNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the SPI0 peripheral."]
    #[inline(always)]
    pub fn spio0reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp0RstN::Spio0reset)
    }
    #[doc = "SPI0 reset de-asserted."]
    #[inline(always)]
    pub fn spio0noreset(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp0RstN::Spio0noreset)
    }
}
#[doc = "I2C reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cRstN {
    #[doc = "0: Resets the I2C peripheral."]
    I2creset = 0,
    #[doc = "1: I2C reset de-asserted."]
    I2cnoreset = 1,
}
impl From<I2cRstN> for bool {
    #[inline(always)]
    fn from(variant: I2cRstN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_RST_N` reader - I2C reset control"]
pub type I2cRstNR = crate::BitReader<I2cRstN>;
impl I2cRstNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cRstN {
        match self.bits {
            false => I2cRstN::I2creset,
            true => I2cRstN::I2cnoreset,
        }
    }
    #[doc = "Resets the I2C peripheral."]
    #[inline(always)]
    pub fn is_i2creset(&self) -> bool {
        *self == I2cRstN::I2creset
    }
    #[doc = "I2C reset de-asserted."]
    #[inline(always)]
    pub fn is_i2cnoreset(&self) -> bool {
        *self == I2cRstN::I2cnoreset
    }
}
#[doc = "Field `I2C_RST_N` writer - I2C reset control"]
pub type I2cRstNW<'a, REG> = crate::BitWriter<'a, REG, I2cRstN>;
impl<'a, REG> I2cRstNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the I2C peripheral."]
    #[inline(always)]
    pub fn i2creset(self) -> &'a mut crate::W<REG> {
        self.variant(I2cRstN::I2creset)
    }
    #[doc = "I2C reset de-asserted."]
    #[inline(always)]
    pub fn i2cnoreset(self) -> &'a mut crate::W<REG> {
        self.variant(I2cRstN::I2cnoreset)
    }
}
#[doc = "SPI1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp1RstN {
    #[doc = "0: Resets the SPI1 peripheral."]
    Spi1reset = 0,
    #[doc = "1: SPI1 reset de-asserted."]
    Spi2noreset = 1,
}
impl From<Ssp1RstN> for bool {
    #[inline(always)]
    fn from(variant: Ssp1RstN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP1_RST_N` reader - SPI1 reset control"]
pub type Ssp1RstNR = crate::BitReader<Ssp1RstN>;
impl Ssp1RstNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp1RstN {
        match self.bits {
            false => Ssp1RstN::Spi1reset,
            true => Ssp1RstN::Spi2noreset,
        }
    }
    #[doc = "Resets the SPI1 peripheral."]
    #[inline(always)]
    pub fn is_spi1reset(&self) -> bool {
        *self == Ssp1RstN::Spi1reset
    }
    #[doc = "SPI1 reset de-asserted."]
    #[inline(always)]
    pub fn is_spi2noreset(&self) -> bool {
        *self == Ssp1RstN::Spi2noreset
    }
}
#[doc = "Field `SSP1_RST_N` writer - SPI1 reset control"]
pub type Ssp1RstNW<'a, REG> = crate::BitWriter<'a, REG, Ssp1RstN>;
impl<'a, REG> Ssp1RstNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the SPI1 peripheral."]
    #[inline(always)]
    pub fn spi1reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1RstN::Spi1reset)
    }
    #[doc = "SPI1 reset de-asserted."]
    #[inline(always)]
    pub fn spi2noreset(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1RstN::Spi2noreset)
    }
}
#[doc = "C_CAN reset control. See Section 3.1 for part specific details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanRstN {
    #[doc = "0: Resets the C_CAN peripheral."]
    Canreset = 0,
    #[doc = "1: C_CAN reset de-asserted."]
    Cannoreset = 1,
}
impl From<CanRstN> for bool {
    #[inline(always)]
    fn from(variant: CanRstN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN_RST_N` reader - C_CAN reset control. See Section 3.1 for part specific details."]
pub type CanRstNR = crate::BitReader<CanRstN>;
impl CanRstNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CanRstN {
        match self.bits {
            false => CanRstN::Canreset,
            true => CanRstN::Cannoreset,
        }
    }
    #[doc = "Resets the C_CAN peripheral."]
    #[inline(always)]
    pub fn is_canreset(&self) -> bool {
        *self == CanRstN::Canreset
    }
    #[doc = "C_CAN reset de-asserted."]
    #[inline(always)]
    pub fn is_cannoreset(&self) -> bool {
        *self == CanRstN::Cannoreset
    }
}
#[doc = "Field `CAN_RST_N` writer - C_CAN reset control. See Section 3.1 for part specific details."]
pub type CanRstNW<'a, REG> = crate::BitWriter<'a, REG, CanRstN>;
impl<'a, REG> CanRstNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the C_CAN peripheral."]
    #[inline(always)]
    pub fn canreset(self) -> &'a mut crate::W<REG> {
        self.variant(CanRstN::Canreset)
    }
    #[doc = "C_CAN reset de-asserted."]
    #[inline(always)]
    pub fn cannoreset(self) -> &'a mut crate::W<REG> {
        self.variant(CanRstN::Cannoreset)
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> Ssp0RstNR {
        Ssp0RstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&self) -> I2cRstNR {
        I2cRstNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> Ssp1RstNR {
        Ssp1RstNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline(always)]
    pub fn can_rst_n(&self) -> CanRstNR {
        CanRstNR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn ssp0_rst_n(&mut self) -> Ssp0RstNW<PresetctrlSpec> {
        Ssp0RstNW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rst_n(&mut self) -> I2cRstNW<PresetctrlSpec> {
        I2cRstNW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn ssp1_rst_n(&mut self) -> Ssp1RstNW<PresetctrlSpec> {
        Ssp1RstNW::new(self, 2)
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline(always)]
    #[must_use]
    pub fn can_rst_n(&mut self) -> CanRstNW<PresetctrlSpec> {
        CanRstNW::new(self, 3)
    }
}
#[doc = "Peripheral reset control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presetctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presetctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlSpec;
impl crate::RegisterSpec for PresetctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrl::R`](R) reader structure"]
impl crate::Readable for PresetctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`presetctrl::W`](W) writer structure"]
impl crate::Writable for PresetctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0"]
impl crate::Resettable for PresetctrlSpec {
    const RESET_VALUE: u32 = 0;
}
