#[doc = "Register `CLKOUTCLKSEL` reader"]
pub type R = crate::R<ClkoutclkselSpec>;
#[doc = "Register `CLKOUTCLKSEL` writer"]
pub type W = crate::W<ClkoutclkselSpec>;
#[doc = "CLKOUT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: IRC oscillator"]
    IrcOscillator = 0,
    #[doc = "1: System oscillator"]
    SystemOscillator = 1,
    #[doc = "2: Watchdog oscillator"]
    WatchdogOscillator = 2,
    #[doc = "3: Main clock"]
    MainClock = 3,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
#[doc = "Field `SEL` reader - CLKOUT clock source"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::IrcOscillator,
            1 => Sel::SystemOscillator,
            2 => Sel::WatchdogOscillator,
            3 => Sel::MainClock,
            _ => unreachable!(),
        }
    }
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == Sel::IrcOscillator
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn is_system_oscillator(&self) -> bool {
        *self == Sel::SystemOscillator
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == Sel::WatchdogOscillator
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == Sel::MainClock
    }
}
#[doc = "Field `SEL` writer - CLKOUT clock source"]
pub type SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::IrcOscillator)
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn system_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SystemOscillator)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::WatchdogOscillator)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainClock)
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<ClkoutclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "CLKOUT clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutclkselSpec;
impl crate::RegisterSpec for ClkoutclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutclksel::R`](R) reader structure"]
impl crate::Readable for ClkoutclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutclksel::W`](W) writer structure"]
impl crate::Writable for ClkoutclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTCLKSEL to value 0"]
impl crate::Resettable for ClkoutclkselSpec {
    const RESET_VALUE: u32 = 0;
}
