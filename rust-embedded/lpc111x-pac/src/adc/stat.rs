#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `DONE` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel n."]
pub type DoneR = crate::FieldReader;
#[doc = "Field `OVERRUN` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel n. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OverrunR = crate::FieldReader;
#[doc = "Field `ADINT` reader - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
pub type AdintR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - These bits mirror the DONE status flags that appear in the result register for each A/D channel n."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel n. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> AdintR {
        AdintR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
