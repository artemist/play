#[doc = "Register `FMSSTOP` reader"]
pub type R = crate::R<FmsstopSpec>;
#[doc = "Register `FMSSTOP` writer"]
pub type W = crate::W<FmsstopSpec>;
#[doc = "Field `STOP` reader - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub type StopR = crate::FieldReader<u32>;
#[doc = "Field `STOP` writer - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Start control bit for signature generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigStart {
    #[doc = "0: Signature generation is stopped"]
    SignatureGeneration = 0,
    #[doc = "1: Initiate signature generation"]
    InitiateSignatureG = 1,
}
impl From<SigStart> for bool {
    #[inline(always)]
    fn from(variant: SigStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIG_START` reader - Start control bit for signature generation."]
pub type SigStartR = crate::BitReader<SigStart>;
impl SigStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SigStart {
        match self.bits {
            false => SigStart::SignatureGeneration,
            true => SigStart::InitiateSignatureG,
        }
    }
    #[doc = "Signature generation is stopped"]
    #[inline(always)]
    pub fn is_signature_generation(&self) -> bool {
        *self == SigStart::SignatureGeneration
    }
    #[doc = "Initiate signature generation"]
    #[inline(always)]
    pub fn is_initiate_signature_g(&self) -> bool {
        *self == SigStart::InitiateSignatureG
    }
}
#[doc = "Field `SIG_START` writer - Start control bit for signature generation."]
pub type SigStartW<'a, REG> = crate::BitWriter<'a, REG, SigStart>;
impl<'a, REG> SigStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Signature generation is stopped"]
    #[inline(always)]
    pub fn signature_generation(self) -> &'a mut crate::W<REG> {
        self.variant(SigStart::SignatureGeneration)
    }
    #[doc = "Initiate signature generation"]
    #[inline(always)]
    pub fn initiate_signature_g(self) -> &'a mut crate::W<REG> {
        self.variant(SigStart::InitiateSignatureG)
    }
}
impl R {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&self) -> SigStartR {
        SigStartR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<FmsstopSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    #[must_use]
    pub fn sig_start(&mut self) -> SigStartW<FmsstopSpec> {
        SigStartW::new(self, 17)
    }
}
#[doc = "Signature stop-address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsstop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmsstop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmsstopSpec;
impl crate::RegisterSpec for FmsstopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsstop::R`](R) reader structure"]
impl crate::Readable for FmsstopSpec {}
#[doc = "`write(|w| ..)` method takes [`fmsstop::W`](W) writer structure"]
impl crate::Writable for FmsstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMSSTOP to value 0"]
impl crate::Resettable for FmsstopSpec {
    const RESET_VALUE: u32 = 0;
}
