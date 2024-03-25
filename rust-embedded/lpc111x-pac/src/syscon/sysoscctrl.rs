#[doc = "Register `SYSOSCCTRL` reader"]
pub type R = crate::R<SysoscctrlSpec>;
#[doc = "Register `SYSOSCCTRL` writer"]
pub type W = crate::W<SysoscctrlSpec>;
#[doc = "Bypass system oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Oscillator is not bypassed."]
    Nobypass = 0,
    #[doc = "1: Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    BypassEnabledPll_ = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Bypass system oscillator"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Nobypass,
            true => Bypass::BypassEnabledPll_,
        }
    }
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn is_nobypass(&self) -> bool {
        *self == Bypass::Nobypass
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    #[inline(always)]
    pub fn is_bypass_enabled_pll_(&self) -> bool {
        *self == Bypass::BypassEnabledPll_
    }
}
#[doc = "Field `BYPASS` writer - Bypass system oscillator"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn nobypass(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Nobypass)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    #[inline(always)]
    pub fn bypass_enabled_pll_(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::BypassEnabledPll_)
    }
}
#[doc = "Determines frequency range for Low-power oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freqrange {
    #[doc = "0: 1 - 20 MHz frequency range."]
    Low = 0,
    #[doc = "1: 15 - 25 MHz frequency range"]
    High = 1,
}
impl From<Freqrange> for bool {
    #[inline(always)]
    fn from(variant: Freqrange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQRANGE` reader - Determines frequency range for Low-power oscillator."]
pub type FreqrangeR = crate::BitReader<Freqrange>;
impl FreqrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freqrange {
        match self.bits {
            false => Freqrange::Low,
            true => Freqrange::High,
        }
    }
    #[doc = "1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Freqrange::Low
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Freqrange::High
    }
}
#[doc = "Field `FREQRANGE` writer - Determines frequency range for Low-power oscillator."]
pub type FreqrangeW<'a, REG> = crate::BitWriter<'a, REG, Freqrange>;
impl<'a, REG> FreqrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Freqrange::Low)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Freqrange::High)
    }
}
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<SysoscctrlSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn freqrange(&mut self) -> FreqrangeW<SysoscctrlSpec> {
        FreqrangeW::new(self, 1)
    }
}
#[doc = "System oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysoscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysoscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysoscctrlSpec;
impl crate::RegisterSpec for SysoscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysoscctrl::R`](R) reader structure"]
impl crate::Readable for SysoscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysoscctrl::W`](W) writer structure"]
impl crate::Writable for SysoscctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCCTRL to value 0"]
impl crate::Resettable for SysoscctrlSpec {
    const RESET_VALUE: u32 = 0;
}
