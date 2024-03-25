#[doc = "Register `SYSAHBCLKDIV` reader"]
pub type R = crate::R<SysahbclkdivSpec>;
#[doc = "Register `SYSAHBCLKDIV` writer"]
pub type W = crate::W<SysahbclkdivSpec>;
#[doc = "Field `DIV` reader - System AHB clock divider values 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - System AHB clock divider values 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - System AHB clock divider values 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System AHB clock divider values 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<SysahbclkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "System AHB clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysahbclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysahbclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysahbclkdivSpec;
impl crate::RegisterSpec for SysahbclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysahbclkdiv::R`](R) reader structure"]
impl crate::Readable for SysahbclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysahbclkdiv::W`](W) writer structure"]
impl crate::Writable for SysahbclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSAHBCLKDIV to value 0x01"]
impl crate::Resettable for SysahbclkdivSpec {
    const RESET_VALUE: u32 = 0x01;
}
