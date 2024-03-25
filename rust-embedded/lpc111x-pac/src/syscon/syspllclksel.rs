#[doc = "Register `SYSPLLCLKSEL` reader"]
pub type R = crate::R<SyspllclkselSpec>;
#[doc = "Register `SYSPLLCLKSEL` writer"]
pub type W = crate::W<SyspllclkselSpec>;
#[doc = "System PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: IRC oscillator"]
    IrcOscillator = 0,
    #[doc = "1: System oscillator"]
    SystemOscillator = 1,
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
#[doc = "Field `SEL` reader - System PLL clock source"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::IrcOscillator,
            1 => Sel::SystemOscillator,
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
}
#[doc = "Field `SEL` writer - System PLL clock source"]
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
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn system_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SystemOscillator)
    }
}
impl R {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<SyspllclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "System PLL clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syspllclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllclkselSpec;
impl crate::RegisterSpec for SyspllclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllclksel::R`](R) reader structure"]
impl crate::Readable for SyspllclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`syspllclksel::W`](W) writer structure"]
impl crate::Writable for SyspllclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLCLKSEL to value 0"]
impl crate::Resettable for SyspllclkselSpec {
    const RESET_VALUE: u32 = 0;
}
