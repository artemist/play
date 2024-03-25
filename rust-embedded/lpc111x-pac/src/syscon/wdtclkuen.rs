#[doc = "Register `WDTCLKUEN` reader"]
pub type R = crate::R<WdtclkuenSpec>;
#[doc = "Register `WDTCLKUEN` writer"]
pub type W = crate::W<WdtclkuenSpec>;
#[doc = "Enable WDT clock source update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena {
    #[doc = "0: No change"]
    NoChange = 0,
    #[doc = "1: Update clock source"]
    UpdateClockSource = 1,
}
impl From<Ena> for bool {
    #[inline(always)]
    fn from(variant: Ena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA` reader - Enable WDT clock source update"]
pub type EnaR = crate::BitReader<Ena>;
impl EnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena {
        match self.bits {
            false => Ena::NoChange,
            true => Ena::UpdateClockSource,
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Ena::NoChange
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn is_update_clock_source(&self) -> bool {
        *self == Ena::UpdateClockSource
    }
}
#[doc = "Field `ENA` writer - Enable WDT clock source update"]
pub type EnaW<'a, REG> = crate::BitWriter<'a, REG, Ena>;
impl<'a, REG> EnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(Ena::NoChange)
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn update_clock_source(self) -> &'a mut crate::W<REG> {
        self.variant(Ena::UpdateClockSource)
    }
}
impl R {
    #[doc = "Bit 0 - Enable WDT clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WDT clock source update"]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> EnaW<WdtclkuenSpec> {
        EnaW::new(self, 0)
    }
}
#[doc = "WDT clock source update enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkuen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkuen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclkuenSpec;
impl crate::RegisterSpec for WdtclkuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkuen::R`](R) reader structure"]
impl crate::Readable for WdtclkuenSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtclkuen::W`](W) writer structure"]
impl crate::Writable for WdtclkuenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCLKUEN to value 0"]
impl crate::Resettable for WdtclkuenSpec {
    const RESET_VALUE: u32 = 0;
}
