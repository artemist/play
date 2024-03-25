#[doc = "Register `WDTCLKSEL` reader"]
pub type R = crate::R<WdtclkselSpec>;
#[doc = "Register `WDTCLKSEL` writer"]
pub type W = crate::W<WdtclkselSpec>;
#[doc = "WDT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: IRC oscillator"]
    IrcOscillator = 0,
    #[doc = "1: Main clock"]
    MainClock = 1,
    #[doc = "2: Watchdog oscillator"]
    WatchdogOscillator = 2,
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
#[doc = "Field `SEL` reader - WDT clock source"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::IrcOscillator,
            1 => Sel::MainClock,
            2 => Sel::WatchdogOscillator,
            _ => unreachable!(),
        }
    }
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == Sel::IrcOscillator
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == Sel::MainClock
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == Sel::WatchdogOscillator
    }
}
#[doc = "Field `SEL` writer - WDT clock source"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel>;
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
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainClock)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::WatchdogOscillator)
    }
}
impl R {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<WdtclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "WDT clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclkselSpec;
impl crate::RegisterSpec for WdtclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclksel::R`](R) reader structure"]
impl crate::Readable for WdtclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtclksel::W`](W) writer structure"]
impl crate::Writable for WdtclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCLKSEL to value 0"]
impl crate::Resettable for WdtclkselSpec {
    const RESET_VALUE: u32 = 0;
}
