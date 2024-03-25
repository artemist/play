#[doc = "Register `PC` reader"]
pub type R = crate::R<PcSpec>;
#[doc = "Register `PC` writer"]
pub type W = crate::W<PcSpec>;
#[doc = "Field `PC` reader - Prescale counter value."]
pub type PcR = crate::FieldReader<u32>;
#[doc = "Field `PC` writer - Prescale counter value."]
pub type PcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PcW<PcSpec> {
        PcW::new(self, 0)
    }
}
#[doc = "Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcSpec;
impl crate::RegisterSpec for PcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PcSpec {}
#[doc = "`write(|w| ..)` method takes [`pc::W`](W) writer structure"]
impl crate::Writable for PcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PcSpec {
    const RESET_VALUE: u32 = 0;
}
