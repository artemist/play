#[doc = "Register `SCLH` reader"]
pub type R = crate::R<SclhSpec>;
#[doc = "Register `SCLH` writer"]
pub type W = crate::W<SclhSpec>;
#[doc = "Field `SCLH` reader - Count for SCL HIGH time period selection."]
pub type SclhR = crate::FieldReader<u16>;
#[doc = "Field `SCLH` writer - Count for SCL HIGH time period selection."]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SclhW<SclhSpec> {
        SclhW::new(self, 0)
    }
}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclhSpec;
impl crate::RegisterSpec for SclhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclh::R`](R) reader structure"]
impl crate::Readable for SclhSpec {}
#[doc = "`write(|w| ..)` method takes [`sclh::W`](W) writer structure"]
impl crate::Writable for SclhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCLH to value 0x04"]
impl crate::Resettable for SclhSpec {
    const RESET_VALUE: u32 = 0x04;
}
