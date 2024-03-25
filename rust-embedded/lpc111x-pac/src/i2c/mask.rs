#[doc = "Register `MASK%s` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK%s` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `MASK` reader - Mask bits."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Mask bits."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Mask bits."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Mask bits."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MaskSpec> {
        MaskW::new(self, 1)
    }
}
#[doc = "I2C Slave address mask register 0. This mask register is associated with I2ADR0 to determine an address match. The mask register has no effect when comparing to the General Call address (0000000).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK%s to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
