#[doc = "Register `FMSW2` reader"]
pub type R = crate::R<Fmsw2Spec>;
#[doc = "Field `SW2_95_64` reader - Word 2 of 128-bit signature (bits 95 to 64)."]
pub type Sw2_95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Word 2 of 128-bit signature (bits 95 to 64)."]
    #[inline(always)]
    pub fn sw2_95_64(&self) -> Sw2_95_64R {
        Sw2_95_64R::new(self.bits)
    }
}
#[doc = "Word 2 \\[95:64\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmsw2Spec;
impl crate::RegisterSpec for Fmsw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsw2::R`](R) reader structure"]
impl crate::Readable for Fmsw2Spec {}
#[doc = "`reset()` method sets FMSW2 to value 0"]
impl crate::Resettable for Fmsw2Spec {
    const RESET_VALUE: u32 = 0;
}
