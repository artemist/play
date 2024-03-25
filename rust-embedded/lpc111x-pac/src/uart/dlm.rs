#[doc = "Register `DLM` reader"]
pub type R = crate::R<DlmSpec>;
#[doc = "Register `DLM` writer"]
pub type W = crate::W<DlmSpec>;
#[doc = "Field `DLMSB` reader - The UART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the UART."]
pub type DlmsbR = crate::FieldReader;
#[doc = "Field `DLMSB` writer - The UART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the UART."]
pub type DlmsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The UART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the UART."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DlmsbR {
        DlmsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The UART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the UART."]
    #[inline(always)]
    #[must_use]
    pub fn dlmsb(&mut self) -> DlmsbW<DlmSpec> {
        DlmsbW::new(self, 0)
    }
}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlmSpec;
impl crate::RegisterSpec for DlmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlm::R`](R) reader structure"]
impl crate::Readable for DlmSpec {}
#[doc = "`write(|w| ..)` method takes [`dlm::W`](W) writer structure"]
impl crate::Writable for DlmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DlmSpec {
    const RESET_VALUE: u32 = 0;
}
