#[doc = "Register `RS485ADRMATCH` reader"]
pub type R = crate::R<Rs485adrmatchSpec>;
#[doc = "Register `RS485ADRMATCH` writer"]
pub type W = crate::W<Rs485adrmatchSpec>;
#[doc = "Field `ADRMATCH` reader - Contains the address match value."]
pub type AdrmatchR = crate::FieldReader;
#[doc = "Field `ADRMATCH` writer - Contains the address match value."]
pub type AdrmatchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    pub fn adrmatch(&self) -> AdrmatchR {
        AdrmatchR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    #[must_use]
    pub fn adrmatch(&mut self) -> AdrmatchW<Rs485adrmatchSpec> {
        AdrmatchW::new(self, 0)
    }
}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485adrmatch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485adrmatch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs485adrmatchSpec;
impl crate::RegisterSpec for Rs485adrmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485adrmatch::R`](R) reader structure"]
impl crate::Readable for Rs485adrmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`rs485adrmatch::W`](W) writer structure"]
impl crate::Writable for Rs485adrmatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS485ADRMATCH to value 0"]
impl crate::Resettable for Rs485adrmatchSpec {
    const RESET_VALUE: u32 = 0;
}
