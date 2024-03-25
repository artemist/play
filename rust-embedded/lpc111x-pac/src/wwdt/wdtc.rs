#[doc = "Register `WDTC` reader"]
pub type R = crate::R<WdtcSpec>;
#[doc = "Register `WDTC` writer"]
pub type W = crate::W<WdtcSpec>;
#[doc = "Field `Count` reader - Watchdog time-out interval."]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `Count` writer - Watchdog time-out interval."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Watchdog time-out interval."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog time-out interval."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<WdtcSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Watchdog timer constant register. This register determines the time-out value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcSpec;
impl crate::RegisterSpec for WdtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtc::R`](R) reader structure"]
impl crate::Readable for WdtcSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtc::W`](W) writer structure"]
impl crate::Writable for WdtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTC to value 0xff"]
impl crate::Resettable for WdtcSpec {
    const RESET_VALUE: u32 = 0xff;
}
