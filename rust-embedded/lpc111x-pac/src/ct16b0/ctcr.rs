#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CtcrSpec>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CtcrSpec>;
#[doc = "Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctm {
    #[doc = "0: Increment on every rising PCLK edge"]
    Timer = 0,
    #[doc = "1: Increment on rising CAP pin edge"]
    CounterRising = 1,
    #[doc = "2: Increment on falling CAP pin edge"]
    CounterFalling = 2,
    #[doc = "3: Increment on both CAP pin edges"]
    CounterBoth = 3,
}
impl From<Ctm> for u8 {
    #[inline(always)]
    fn from(variant: Ctm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctm {
    type Ux = u8;
}
#[doc = "Field `CTM` reader - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
pub type CtmR = crate::FieldReader<Ctm>;
impl CtmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctm {
        match self.bits {
            0 => Ctm::Timer,
            1 => Ctm::CounterRising,
            2 => Ctm::CounterFalling,
            3 => Ctm::CounterBoth,
            _ => unreachable!(),
        }
    }
    #[doc = "Increment on every rising PCLK edge"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Ctm::Timer
    }
    #[doc = "Increment on rising CAP pin edge"]
    #[inline(always)]
    pub fn is_counter_rising(&self) -> bool {
        *self == Ctm::CounterRising
    }
    #[doc = "Increment on falling CAP pin edge"]
    #[inline(always)]
    pub fn is_counter_falling(&self) -> bool {
        *self == Ctm::CounterFalling
    }
    #[doc = "Increment on both CAP pin edges"]
    #[inline(always)]
    pub fn is_counter_both(&self) -> bool {
        *self == Ctm::CounterBoth
    }
}
#[doc = "Field `CTM` writer - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
pub type CtmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ctm>;
impl<'a, REG> CtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment on every rising PCLK edge"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::Timer)
    }
    #[doc = "Increment on rising CAP pin edge"]
    #[inline(always)]
    pub fn counter_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::CounterRising)
    }
    #[doc = "Increment on falling CAP pin edge"]
    #[inline(always)]
    pub fn counter_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::CounterFalling)
    }
    #[doc = "Increment on both CAP pin edges"]
    #[inline(always)]
    pub fn counter_both(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::CounterBoth)
    }
}
#[doc = "Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cis {
    #[doc = "0: CT16Bn_CAP0"]
    Ct16bnCap0 = 0,
    #[doc = "1: CT16Bn_CAP1"]
    Ct16bnCap1 = 1,
}
impl From<Cis> for u8 {
    #[inline(always)]
    fn from(variant: Cis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cis {
    type Ux = u8;
}
#[doc = "Field `CIS` reader - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub type CisR = crate::FieldReader<Cis>;
impl CisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cis> {
        match self.bits {
            0 => Some(Cis::Ct16bnCap0),
            1 => Some(Cis::Ct16bnCap1),
            _ => None,
        }
    }
    #[doc = "CT16Bn_CAP0"]
    #[inline(always)]
    pub fn is_ct16bn_cap0(&self) -> bool {
        *self == Cis::Ct16bnCap0
    }
    #[doc = "CT16Bn_CAP1"]
    #[inline(always)]
    pub fn is_ct16bn_cap1(&self) -> bool {
        *self == Cis::Ct16bnCap1
    }
}
#[doc = "Field `CIS` writer - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub type CisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cis>;
impl<'a, REG> CisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CT16Bn_CAP0"]
    #[inline(always)]
    pub fn ct16bn_cap0(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::Ct16bnCap0)
    }
    #[doc = "CT16Bn_CAP1"]
    #[inline(always)]
    pub fn ct16bn_cap1(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::Ct16bnCap1)
    }
}
#[doc = "Field `ENCC` reader - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type EnccR = crate::BitReader;
#[doc = "Field `ENCC` writer - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type EnccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selcc {
    #[doc = "0: Rising Edge of CAP0 clears the timer"]
    Cap0Rising = 0,
    #[doc = "1: Falling Edge of CAP0 clears the timer"]
    Cap0Falling = 1,
    #[doc = "2: Rising Edge of CAP1 clears the timer"]
    Cap1Rising = 2,
    #[doc = "3: Falling Edge of CAP1 clears the timer"]
    Cap1Falling = 3,
}
impl From<Selcc> for u8 {
    #[inline(always)]
    fn from(variant: Selcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selcc {
    type Ux = u8;
}
#[doc = "Field `SELCC` reader - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
pub type SelccR = crate::FieldReader<Selcc>;
impl SelccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selcc> {
        match self.bits {
            0 => Some(Selcc::Cap0Rising),
            1 => Some(Selcc::Cap0Falling),
            2 => Some(Selcc::Cap1Rising),
            3 => Some(Selcc::Cap1Falling),
            _ => None,
        }
    }
    #[doc = "Rising Edge of CAP0 clears the timer"]
    #[inline(always)]
    pub fn is_cap0_rising(&self) -> bool {
        *self == Selcc::Cap0Rising
    }
    #[doc = "Falling Edge of CAP0 clears the timer"]
    #[inline(always)]
    pub fn is_cap0_falling(&self) -> bool {
        *self == Selcc::Cap0Falling
    }
    #[doc = "Rising Edge of CAP1 clears the timer"]
    #[inline(always)]
    pub fn is_cap1_rising(&self) -> bool {
        *self == Selcc::Cap1Rising
    }
    #[doc = "Falling Edge of CAP1 clears the timer"]
    #[inline(always)]
    pub fn is_cap1_falling(&self) -> bool {
        *self == Selcc::Cap1Falling
    }
}
#[doc = "Field `SELCC` writer - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
pub type SelccW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selcc>;
impl<'a, REG> SelccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising Edge of CAP0 clears the timer"]
    #[inline(always)]
    pub fn cap0_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Cap0Rising)
    }
    #[doc = "Falling Edge of CAP0 clears the timer"]
    #[inline(always)]
    pub fn cap0_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Cap0Falling)
    }
    #[doc = "Rising Edge of CAP1 clears the timer"]
    #[inline(always)]
    pub fn cap1_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Cap1Rising)
    }
    #[doc = "Falling Edge of CAP1 clears the timer"]
    #[inline(always)]
    pub fn cap1_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Cap1Falling)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
    #[inline(always)]
    pub fn ctm(&self) -> CtmR {
        CtmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    pub fn cis(&self) -> CisR {
        CisR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> EnccR {
        EnccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline(always)]
    pub fn selcc(&self) -> SelccR {
        SelccR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
    #[inline(always)]
    #[must_use]
    pub fn ctm(&mut self) -> CtmW<CtcrSpec> {
        CtmW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    #[must_use]
    pub fn cis(&mut self) -> CisW<CtcrSpec> {
        CisW::new(self, 2)
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    #[must_use]
    pub fn encc(&mut self) -> EnccW<CtcrSpec> {
        EnccW::new(self, 4)
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline(always)]
    #[must_use]
    pub fn selcc(&mut self) -> SelccW<CtcrSpec> {
        SelccW::new(self, 5)
    }
}
#[doc = "Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcrSpec;
impl crate::RegisterSpec for CtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctcr::R`](R) reader structure"]
impl crate::Readable for CtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctcr::W`](W) writer structure"]
impl crate::Writable for CtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CtcrSpec {
    const RESET_VALUE: u32 = 0;
}
