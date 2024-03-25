#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `PR` reader - Prescale max value."]
pub type PrR = crate::FieldReader<u16>;
#[doc = "Field `PR` writer - Prescale max value."]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Prescale max value."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescale max value."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<PrSpec> {
        PrW::new(self, 0)
    }
}
#[doc = "Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {
    const RESET_VALUE: u32 = 0;
}
