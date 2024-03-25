#[doc = "Register `CR%s` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Field `CAP` reader - Timer counter capture value."]
pub type CapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter capture value."]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(self.bits)
    }
}
#[doc = "Capture Register (CR). CR is loaded with the value of TC when there is an event on the CT16Bn_CAPm input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`reset()` method sets CR%s to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
