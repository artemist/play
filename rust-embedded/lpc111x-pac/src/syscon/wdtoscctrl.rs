#[doc = "Register `WDTOSCCTRL` reader"]
pub type R = crate::R<WdtoscctrlSpec>;
#[doc = "Register `WDTOSCCTRL` writer"]
pub type W = crate::W<WdtoscctrlSpec>;
#[doc = "Field `DIVSEL` reader - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
pub type DivselR = crate::FieldReader;
#[doc = "Field `DIVSEL` writer - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
pub type DivselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Select watchdog oscillator analog output frequency (Fclkana).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freqsel {
    #[doc = "1: 0.6 MHz"]
    _0_6Mhz = 1,
    #[doc = "2: 1.05 MHz"]
    _1_05Mhz = 2,
    #[doc = "3: 1.4 MHz"]
    _1_4Mhz = 3,
    #[doc = "4: 1.75 MHz"]
    _1_75Mhz = 4,
    #[doc = "5: 2.1 MHz"]
    _2_1Mhz = 5,
    #[doc = "6: 2.4 MHz"]
    _2_4Mhz = 6,
    #[doc = "7: 2.7 MHz"]
    _2_7Mhz = 7,
    #[doc = "8: 3.0 MHz"]
    _3_0Mhz = 8,
    #[doc = "9: 3.25 MHz"]
    _3_25Mhz = 9,
    #[doc = "10: 3.5 MHz"]
    _3_5Mhz = 10,
    #[doc = "11: 3.75 MHz"]
    _3_75Mhz = 11,
    #[doc = "12: 4.0 MHz"]
    _4_0Mhz = 12,
    #[doc = "13: 4.2 MHz"]
    _4_2Mhz = 13,
    #[doc = "14: 4.4 MHz"]
    _4_4Mhz = 14,
    #[doc = "15: 4.6 MHz"]
    _4_6Mhz = 15,
}
impl From<Freqsel> for u8 {
    #[inline(always)]
    fn from(variant: Freqsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freqsel {
    type Ux = u8;
}
#[doc = "Field `FREQSEL` reader - Select watchdog oscillator analog output frequency (Fclkana)."]
pub type FreqselR = crate::FieldReader<Freqsel>;
impl FreqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freqsel> {
        match self.bits {
            1 => Some(Freqsel::_0_6Mhz),
            2 => Some(Freqsel::_1_05Mhz),
            3 => Some(Freqsel::_1_4Mhz),
            4 => Some(Freqsel::_1_75Mhz),
            5 => Some(Freqsel::_2_1Mhz),
            6 => Some(Freqsel::_2_4Mhz),
            7 => Some(Freqsel::_2_7Mhz),
            8 => Some(Freqsel::_3_0Mhz),
            9 => Some(Freqsel::_3_25Mhz),
            10 => Some(Freqsel::_3_5Mhz),
            11 => Some(Freqsel::_3_75Mhz),
            12 => Some(Freqsel::_4_0Mhz),
            13 => Some(Freqsel::_4_2Mhz),
            14 => Some(Freqsel::_4_4Mhz),
            15 => Some(Freqsel::_4_6Mhz),
            _ => None,
        }
    }
    #[doc = "0.6 MHz"]
    #[inline(always)]
    pub fn is_0_6_mhz(&self) -> bool {
        *self == Freqsel::_0_6Mhz
    }
    #[doc = "1.05 MHz"]
    #[inline(always)]
    pub fn is_1_05_mhz(&self) -> bool {
        *self == Freqsel::_1_05Mhz
    }
    #[doc = "1.4 MHz"]
    #[inline(always)]
    pub fn is_1_4_mhz(&self) -> bool {
        *self == Freqsel::_1_4Mhz
    }
    #[doc = "1.75 MHz"]
    #[inline(always)]
    pub fn is_1_75_mhz(&self) -> bool {
        *self == Freqsel::_1_75Mhz
    }
    #[doc = "2.1 MHz"]
    #[inline(always)]
    pub fn is_2_1_mhz(&self) -> bool {
        *self == Freqsel::_2_1Mhz
    }
    #[doc = "2.4 MHz"]
    #[inline(always)]
    pub fn is_2_4_mhz(&self) -> bool {
        *self == Freqsel::_2_4Mhz
    }
    #[doc = "2.7 MHz"]
    #[inline(always)]
    pub fn is_2_7_mhz(&self) -> bool {
        *self == Freqsel::_2_7Mhz
    }
    #[doc = "3.0 MHz"]
    #[inline(always)]
    pub fn is_3_0_mhz(&self) -> bool {
        *self == Freqsel::_3_0Mhz
    }
    #[doc = "3.25 MHz"]
    #[inline(always)]
    pub fn is_3_25_mhz(&self) -> bool {
        *self == Freqsel::_3_25Mhz
    }
    #[doc = "3.5 MHz"]
    #[inline(always)]
    pub fn is_3_5_mhz(&self) -> bool {
        *self == Freqsel::_3_5Mhz
    }
    #[doc = "3.75 MHz"]
    #[inline(always)]
    pub fn is_3_75_mhz(&self) -> bool {
        *self == Freqsel::_3_75Mhz
    }
    #[doc = "4.0 MHz"]
    #[inline(always)]
    pub fn is_4_0_mhz(&self) -> bool {
        *self == Freqsel::_4_0Mhz
    }
    #[doc = "4.2 MHz"]
    #[inline(always)]
    pub fn is_4_2_mhz(&self) -> bool {
        *self == Freqsel::_4_2Mhz
    }
    #[doc = "4.4 MHz"]
    #[inline(always)]
    pub fn is_4_4_mhz(&self) -> bool {
        *self == Freqsel::_4_4Mhz
    }
    #[doc = "4.6 MHz"]
    #[inline(always)]
    pub fn is_4_6_mhz(&self) -> bool {
        *self == Freqsel::_4_6Mhz
    }
}
#[doc = "Field `FREQSEL` writer - Select watchdog oscillator analog output frequency (Fclkana)."]
pub type FreqselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Freqsel>;
impl<'a, REG> FreqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.6 MHz"]
    #[inline(always)]
    pub fn _0_6_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_0_6Mhz)
    }
    #[doc = "1.05 MHz"]
    #[inline(always)]
    pub fn _1_05_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_1_05Mhz)
    }
    #[doc = "1.4 MHz"]
    #[inline(always)]
    pub fn _1_4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_1_4Mhz)
    }
    #[doc = "1.75 MHz"]
    #[inline(always)]
    pub fn _1_75_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_1_75Mhz)
    }
    #[doc = "2.1 MHz"]
    #[inline(always)]
    pub fn _2_1_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_2_1Mhz)
    }
    #[doc = "2.4 MHz"]
    #[inline(always)]
    pub fn _2_4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_2_4Mhz)
    }
    #[doc = "2.7 MHz"]
    #[inline(always)]
    pub fn _2_7_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_2_7Mhz)
    }
    #[doc = "3.0 MHz"]
    #[inline(always)]
    pub fn _3_0_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_3_0Mhz)
    }
    #[doc = "3.25 MHz"]
    #[inline(always)]
    pub fn _3_25_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_3_25Mhz)
    }
    #[doc = "3.5 MHz"]
    #[inline(always)]
    pub fn _3_5_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_3_5Mhz)
    }
    #[doc = "3.75 MHz"]
    #[inline(always)]
    pub fn _3_75_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_3_75Mhz)
    }
    #[doc = "4.0 MHz"]
    #[inline(always)]
    pub fn _4_0_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_4_0Mhz)
    }
    #[doc = "4.2 MHz"]
    #[inline(always)]
    pub fn _4_2_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_4_2Mhz)
    }
    #[doc = "4.4 MHz"]
    #[inline(always)]
    pub fn _4_4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_4_4Mhz)
    }
    #[doc = "4.6 MHz"]
    #[inline(always)]
    pub fn _4_6_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freqsel::_4_6Mhz)
    }
}
impl R {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    pub fn divsel(&self) -> DivselR {
        DivselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    pub fn freqsel(&self) -> FreqselR {
        FreqselR::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    #[must_use]
    pub fn divsel(&mut self) -> DivselW<WdtoscctrlSpec> {
        DivselW::new(self, 0)
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    #[must_use]
    pub fn freqsel(&mut self) -> FreqselW<WdtoscctrlSpec> {
        FreqselW::new(self, 5)
    }
}
#[doc = "Watchdog oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtoscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtoscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtoscctrlSpec;
impl crate::RegisterSpec for WdtoscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtoscctrl::R`](R) reader structure"]
impl crate::Readable for WdtoscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtoscctrl::W`](W) writer structure"]
impl crate::Writable for WdtoscctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTOSCCTRL to value 0"]
impl crate::Resettable for WdtoscctrlSpec {
    const RESET_VALUE: u32 = 0;
}
