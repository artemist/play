#[doc = "Register `FMSW1` reader"]
pub type R = crate::R<Fmsw1Spec>;
#[doc = "Field `SW1_63_32` reader - Word 1 of 128-bit signature (bits 63 to 32)."]
pub type Sw1_63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Word 1 of 128-bit signature (bits 63 to 32)."]
    #[inline(always)]
    pub fn sw1_63_32(&self) -> Sw1_63_32R {
        Sw1_63_32R::new(self.bits)
    }
}
#[doc = "Word 1 \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmsw1Spec;
impl crate::RegisterSpec for Fmsw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsw1::R`](R) reader structure"]
impl crate::Readable for Fmsw1Spec {}
#[doc = "`reset()` method sets FMSW1 to value 0"]
impl crate::Resettable for Fmsw1Spec {
    const RESET_VALUE: u32 = 0;
}
