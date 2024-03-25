#[doc = "Register `RBR` reader"]
pub type R = crate::R<RbrSpec>;
#[doc = "Field `RBR` reader - The UART Receiver Buffer Register contains the oldest received byte in the UART RX FIFO."]
pub type RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The UART Receiver Buffer Register contains the oldest received byte in the UART RX FIFO."]
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R). WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrSpec;
impl crate::RegisterSpec for RbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RbrSpec {}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RbrSpec {
    const RESET_VALUE: u32 = 0;
}
