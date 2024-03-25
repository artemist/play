#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RORIC` writer - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt."]
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a timeout period interrupt. The timeout period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RoricW<IcrSpec> {
        RoricW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a timeout period interrupt. The timeout period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 1)
    }
}
#[doc = "SSPICR Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
