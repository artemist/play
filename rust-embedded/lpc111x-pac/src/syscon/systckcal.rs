#[doc = "Register `SYSTCKCAL` reader"]
pub type R = crate::R<SystckcalSpec>;
#[doc = "Register `SYSTCKCAL` writer"]
pub type W = crate::W<SystckcalSpec>;
#[doc = "Field `CAL` reader - System tick timer calibration value"]
pub type CalR = crate::FieldReader<u32>;
#[doc = "Field `CAL` writer - System tick timer calibration value"]
pub type CalW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CalW<SystckcalSpec> {
        CalW::new(self, 0)
    }
}
#[doc = "System tick counter calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systckcal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systckcal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystckcalSpec;
impl crate::RegisterSpec for SystckcalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systckcal::R`](R) reader structure"]
impl crate::Readable for SystckcalSpec {}
#[doc = "`write(|w| ..)` method takes [`systckcal::W`](W) writer structure"]
impl crate::Writable for SystckcalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTCKCAL to value 0x04"]
impl crate::Resettable for SystckcalSpec {
    const RESET_VALUE: u32 = 0x04;
}
