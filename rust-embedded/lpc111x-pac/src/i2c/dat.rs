#[doc = "Register `DAT` reader"]
pub type R = crate::R<DatSpec>;
#[doc = "Register `DAT` writer"]
pub type W = crate::W<DatSpec>;
#[doc = "Field `Data` reader - This register holds data values that have been received or are to be transmitted."]
pub type DataR = crate::FieldReader;
#[doc = "Field `Data` writer - This register holds data values that have been received or are to be transmitted."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatSpec;
impl crate::RegisterSpec for DatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat::R`](R) reader structure"]
impl crate::Readable for DatSpec {}
#[doc = "`write(|w| ..)` method takes [`dat::W`](W) writer structure"]
impl crate::Writable for DatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAT to value 0"]
impl crate::Resettable for DatSpec {
    const RESET_VALUE: u32 = 0;
}
