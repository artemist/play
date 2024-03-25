#[doc = "Register `WDWINDOW` reader"]
pub type R = crate::R<WdwindowSpec>;
#[doc = "Register `WDWINDOW` writer"]
pub type W = crate::W<WdwindowSpec>;
#[doc = "Field `WINDOW` reader - Watchdog window value."]
pub type WindowR = crate::FieldReader<u32>;
#[doc = "Field `WINDOW` writer - Watchdog window value."]
pub type WindowW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WindowW<WdwindowSpec> {
        WindowW::new(self, 0)
    }
}
#[doc = "Watchdog Window compare value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdwindow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdwindow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdwindowSpec;
impl crate::RegisterSpec for WdwindowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdwindow::R`](R) reader structure"]
impl crate::Readable for WdwindowSpec {}
#[doc = "`write(|w| ..)` method takes [`wdwindow::W`](W) writer structure"]
impl crate::Writable for WdwindowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDWINDOW to value 0x00ff_ffff"]
impl crate::Resettable for WdwindowSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
