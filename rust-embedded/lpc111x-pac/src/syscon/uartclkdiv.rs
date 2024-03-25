#[doc = "Register `UARTCLKDIV` reader"]
pub type R = crate::R<UartclkdivSpec>;
#[doc = "Register `UARTCLKDIV` writer"]
pub type W = crate::W<UartclkdivSpec>;
#[doc = "Field `DIV` reader - UART_PCLK clock divider values 0: Disable UART_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - UART_PCLK clock divider values 0: Disable UART_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART_PCLK clock divider values 0: Disable UART_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART_PCLK clock divider values 0: Disable UART_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<UartclkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "UART clock divder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartclkdivSpec;
impl crate::RegisterSpec for UartclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartclkdiv::R`](R) reader structure"]
impl crate::Readable for UartclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`uartclkdiv::W`](W) writer structure"]
impl crate::Writable for UartclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTCLKDIV to value 0"]
impl crate::Resettable for UartclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
