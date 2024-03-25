#[doc = "Register `FMSTAT` reader"]
pub type R = crate::R<FmstatSpec>;
#[doc = "Field `SIG_DONE` reader - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
pub type SigDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline(always)]
    pub fn sig_done(&self) -> SigDoneR {
        SigDoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Signature generation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmstatSpec;
impl crate::RegisterSpec for FmstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmstat::R`](R) reader structure"]
impl crate::Readable for FmstatSpec {}
#[doc = "`reset()` method sets FMSTAT to value 0"]
impl crate::Resettable for FmstatSpec {
    const RESET_VALUE: u32 = 0;
}
