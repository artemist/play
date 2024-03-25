#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RORIM` reader - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RorimR = crate::BitReader;
#[doc = "Field `RORIM` writer - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RorimW<ImscSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR x \\[SCR+1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImscSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImscSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImscSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "Interrupt Mask Set and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {
    const RESET_VALUE: u32 = 0;
}
