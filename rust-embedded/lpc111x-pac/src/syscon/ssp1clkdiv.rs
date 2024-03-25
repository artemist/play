#[doc = "Register `SSP1CLKDIV` reader"]
pub type R = crate::R<Ssp1clkdivSpec>;
#[doc = "Register `SSP1CLKDIV` writer"]
pub type W = crate::W<Ssp1clkdivSpec>;
#[doc = "Field `DIV` reader - SPI1_PCLK clock divider values 0: Disable SPI1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - SPI1_PCLK clock divider values 0: Disable SPI1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI1_PCLK clock divider values 0: Disable SPI1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI1_PCLK clock divider values 0: Disable SPI1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<Ssp1clkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "SPI1 clock divder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp1clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp1clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssp1clkdivSpec;
impl crate::RegisterSpec for Ssp1clkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp1clkdiv::R`](R) reader structure"]
impl crate::Readable for Ssp1clkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp1clkdiv::W`](W) writer structure"]
impl crate::Writable for Ssp1clkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSP1CLKDIV to value 0"]
impl crate::Resettable for Ssp1clkdivSpec {
    const RESET_VALUE: u32 = 0;
}
