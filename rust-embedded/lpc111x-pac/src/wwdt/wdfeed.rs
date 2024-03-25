#[doc = "Register `WDFEED` writer"]
pub type W = crate::W<WdfeedSpec>;
#[doc = "Field `Feed` writer - Feed value should be 0xAA followed by 0x55."]
pub type FeedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    #[must_use]
    pub fn feed(&mut self) -> FeedW<WdfeedSpec> {
        FeedW::new(self, 0)
    }
}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdfeed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdfeedSpec;
impl crate::RegisterSpec for WdfeedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdfeed::W`](W) writer structure"]
impl crate::Writable for WdfeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDFEED to value 0"]
impl crate::Resettable for WdfeedSpec {
    const RESET_VALUE: u32 = 0;
}
