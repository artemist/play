#[doc = "Register `WDTV` reader"]
pub type R = crate::R<WdtvSpec>;
#[doc = "Field `Count` reader - Counter timer value."]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Counter timer value."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtvSpec;
impl crate::RegisterSpec for WdtvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtv::R`](R) reader structure"]
impl crate::Readable for WdtvSpec {}
#[doc = "`reset()` method sets WDTV to value 0xff"]
impl crate::Resettable for WdtvSpec {
    const RESET_VALUE: u32 = 0xff;
}
