#[doc = "Register `FMSW0` reader"]
pub type R = crate::R<Fmsw0Spec>;
#[doc = "Field `SW0_31_0` reader - Word 0 of 128-bit signature (bits 31 to 0)."]
pub type Sw0_31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Word 0 of 128-bit signature (bits 31 to 0)."]
    #[inline(always)]
    pub fn sw0_31_0(&self) -> Sw0_31_0R {
        Sw0_31_0R::new(self.bits)
    }
}
#[doc = "Word 0 \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmsw0Spec;
impl crate::RegisterSpec for Fmsw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsw0::R`](R) reader structure"]
impl crate::Readable for Fmsw0Spec {}
#[doc = "`reset()` method sets FMSW0 to value 0"]
impl crate::Resettable for Fmsw0Spec {
    const RESET_VALUE: u32 = 0;
}
