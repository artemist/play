#[doc = "Register `MAINCLKSEL` reader"]
pub type R = crate::R<MainclkselSpec>;
#[doc = "Register `MAINCLKSEL` writer"]
pub type W = crate::W<MainclkselSpec>;
#[doc = "Clock source for main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: IRC oscillator"]
    IrcOscillator = 0,
    #[doc = "1: Input clock to system PLL"]
    InputClockToSyste = 1,
    #[doc = "2: WDT oscillator"]
    WdtOscillator = 2,
    #[doc = "3: System PLL clock out"]
    SystemPllClockOut = 3,
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
#[doc = "Field `SEL` reader - Clock source for main clock"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::IrcOscillator,
            1 => Sel::InputClockToSyste,
            2 => Sel::WdtOscillator,
            3 => Sel::SystemPllClockOut,
            _ => unreachable!(),
        }
    }
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == Sel::IrcOscillator
    }
    #[doc = "Input clock to system PLL"]
    #[inline(always)]
    pub fn is_input_clock_to_syste(&self) -> bool {
        *self == Sel::InputClockToSyste
    }
    #[doc = "WDT oscillator"]
    #[inline(always)]
    pub fn is_wdt_oscillator(&self) -> bool {
        *self == Sel::WdtOscillator
    }
    #[doc = "System PLL clock out"]
    #[inline(always)]
    pub fn is_system_pll_clock_out(&self) -> bool {
        *self == Sel::SystemPllClockOut
    }
}
#[doc = "Field `SEL` writer - Clock source for main clock"]
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
    #[doc = "Input clock to system PLL"]
    #[inline(always)]
    pub fn input_clock_to_syste(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::InputClockToSyste)
    }
    #[doc = "WDT oscillator"]
    #[inline(always)]
    pub fn wdt_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::WdtOscillator)
    }
    #[doc = "System PLL clock out"]
    #[inline(always)]
    pub fn system_pll_clock_out(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SystemPllClockOut)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<MainclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "Main clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mainclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mainclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclkselSpec;
impl crate::RegisterSpec for MainclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclksel::R`](R) reader structure"]
impl crate::Readable for MainclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`mainclksel::W`](W) writer structure"]
impl crate::Writable for MainclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAINCLKSEL to value 0"]
impl crate::Resettable for MainclkselSpec {
    const RESET_VALUE: u32 = 0;
}
