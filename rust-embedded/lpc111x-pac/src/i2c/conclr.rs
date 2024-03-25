#[doc = "Register `CONCLR` writer"]
pub type W = crate::W<ConclrSpec>;
#[doc = "Field `AAC` writer - Assert acknowledge Clear bit."]
pub type AacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIC` writer - I2C interrupt Clear bit."]
pub type SicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAC` writer - START flag Clear bit."]
pub type StacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2ENC` writer - I2C interface Disable bit."]
pub type I2encW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Assert acknowledge Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn aac(&mut self) -> AacW<ConclrSpec> {
        AacW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C interrupt Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn sic(&mut self) -> SicW<ConclrSpec> {
        SicW::new(self, 3)
    }
    #[doc = "Bit 5 - START flag Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn stac(&mut self) -> StacW<ConclrSpec> {
        StacW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C interface Disable bit."]
    #[inline(always)]
    #[must_use]
    pub fn i2enc(&mut self) -> I2encW<ConclrSpec> {
        I2encW::new(self, 6)
    }
}
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConclrSpec;
impl crate::RegisterSpec for ConclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`conclr::W`](W) writer structure"]
impl crate::Writable for ConclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONCLR to value 0"]
impl crate::Resettable for ConclrSpec {
    const RESET_VALUE: u32 = 0;
}
