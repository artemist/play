#[doc = "Register `CLKOUTCLKDIV` reader"]
pub type R = crate::R<ClkoutclkdivSpec>;
#[doc = "Register `CLKOUTCLKDIV` writer"]
pub type W = crate::W<ClkoutclkdivSpec>;
#[doc = "Field `DIV` reader - Clock output divider values 0: Disable CLKOUT. 1: Divide by 1. to 255: Divide by 255."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock output divider values 0: Disable CLKOUT. 1: Divide by 1. to 255: Divide by 255."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock output divider values 0: Disable CLKOUT. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock output divider values 0: Disable CLKOUT. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<ClkoutclkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "CLKOUT clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutclkdivSpec;
impl crate::RegisterSpec for ClkoutclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutclkdiv::R`](R) reader structure"]
impl crate::Readable for ClkoutclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutclkdiv::W`](W) writer structure"]
impl crate::Writable for ClkoutclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTCLKDIV to value 0"]
impl crate::Resettable for ClkoutclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
