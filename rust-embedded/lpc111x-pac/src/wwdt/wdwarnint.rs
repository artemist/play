#[doc = "Register `WDWARNINT` reader"]
pub type R = crate::R<WdwarnintSpec>;
#[doc = "Register `WDWARNINT` writer"]
pub type W = crate::W<WdwarnintSpec>;
#[doc = "Field `WARNINT` reader - Watchdog warning interrupt compare value."]
pub type WarnintR = crate::FieldReader<u16>;
#[doc = "Field `WARNINT` writer - Watchdog warning interrupt compare value."]
pub type WarnintW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&self) -> WarnintR {
        WarnintR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    #[must_use]
    pub fn warnint(&mut self) -> WarnintW<WdwarnintSpec> {
        WarnintW::new(self, 0)
    }
}
#[doc = "Watchdog Warning Interrupt compare value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdwarnint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdwarnint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdwarnintSpec;
impl crate::RegisterSpec for WdwarnintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdwarnint::R`](R) reader structure"]
impl crate::Readable for WdwarnintSpec {}
#[doc = "`write(|w| ..)` method takes [`wdwarnint::W`](W) writer structure"]
impl crate::Writable for WdwarnintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDWARNINT to value 0"]
impl crate::Resettable for WdwarnintSpec {
    const RESET_VALUE: u32 = 0;
}
