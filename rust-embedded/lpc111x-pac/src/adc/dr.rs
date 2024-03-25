#[doc = "Register `DR%s` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR%s` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `V_VREF` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin, divided by the voltage on the VREF pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VREF, while 0x3FF indicates that the voltage on AD input was close to, equal to, or greater than that on VREF."]
pub type VVrefR = crate::FieldReader<u16>;
#[doc = "Field `V_VREF` writer - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin, divided by the voltage on the VREF pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VREF, while 0x3FF indicates that the voltage on AD input was close to, equal to, or greater than that on VREF."]
pub type VVrefW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits.This bit is cleared by reading this register."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits.This bit is cleared by reading this register."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin, divided by the voltage on the VREF pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VREF, while 0x3FF indicates that the voltage on AD input was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    pub fn v_vref(&self) -> VVrefR {
        VVrefR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits.This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin, divided by the voltage on the VREF pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VREF, while 0x3FF indicates that the voltage on AD input was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    #[must_use]
    pub fn v_vref(&mut self) -> VVrefW<DrSpec> {
        VVrefW::new(self, 6)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits.This bit is cleared by reading this register."]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OverrunW<DrSpec> {
        OverrunW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<DrSpec> {
        DoneW::new(self, 31)
    }
}
#[doc = "A/D Channel n Data Register. This register contains the result of the most recent conversion completed on channel n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR%s to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
