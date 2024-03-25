#[doc = "Register `FMSSTART` reader"]
pub type R = crate::R<FmsstartSpec>;
#[doc = "Register `FMSSTART` writer"]
pub type W = crate::W<FmsstartSpec>;
#[doc = "Field `START` reader - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub type StartR = crate::FieldReader<u32>;
#[doc = "Field `START` writer - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<FmsstartSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Signature start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsstart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmsstart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmsstartSpec;
impl crate::RegisterSpec for FmsstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsstart::R`](R) reader structure"]
impl crate::Readable for FmsstartSpec {}
#[doc = "`write(|w| ..)` method takes [`fmsstart::W`](W) writer structure"]
impl crate::Writable for FmsstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMSSTART to value 0"]
impl crate::Resettable for FmsstartSpec {
    const RESET_VALUE: u32 = 0;
}
