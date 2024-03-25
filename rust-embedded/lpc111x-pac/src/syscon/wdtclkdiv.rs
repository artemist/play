#[doc = "Register `WDTCLKDIV` reader"]
pub type R = crate::R<WdtclkdivSpec>;
#[doc = "Register `WDTCLKDIV` writer"]
pub type W = crate::W<WdtclkdivSpec>;
#[doc = "Field `DIV` reader - WDT clock divider values 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - WDT clock divider values 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - WDT clock divider values 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT clock divider values 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<WdtclkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "WDT clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclkdivSpec;
impl crate::RegisterSpec for WdtclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkdiv::R`](R) reader structure"]
impl crate::Readable for WdtclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtclkdiv::W`](W) writer structure"]
impl crate::Writable for WdtclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCLKDIV to value 0"]
impl crate::Resettable for WdtclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
