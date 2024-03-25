#[doc = "Register `FMSTATCLR` writer"]
pub type W = crate::W<FmstatclrSpec>;
#[doc = "Field `SIG_DONE_CLR` writer - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
pub type SigDoneClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
    #[inline(always)]
    #[must_use]
    pub fn sig_done_clr(&mut self) -> SigDoneClrW<FmstatclrSpec> {
        SigDoneClrW::new(self, 2)
    }
}
#[doc = "Signature generation status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmstatclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmstatclrSpec;
impl crate::RegisterSpec for FmstatclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmstatclr::W`](W) writer structure"]
impl crate::Writable for FmstatclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMSTATCLR to value 0"]
impl crate::Resettable for FmstatclrSpec {
    const RESET_VALUE: u32 = 0;
}
