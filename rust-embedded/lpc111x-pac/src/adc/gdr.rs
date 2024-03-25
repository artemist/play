#[doc = "Register `GDR` reader"]
pub type R = crate::R<GdrSpec>;
#[doc = "Register `GDR` writer"]
pub type W = crate::W<GdrSpec>;
#[doc = "Field `V_VREF` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
pub type VVrefR = crate::FieldReader<u16>;
#[doc = "Field `V_VREF` writer - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
pub type VVrefW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CHN` reader - These bits contain the channel from which the result bits V_VREF were converted."]
pub type ChnR = crate::FieldReader;
#[doc = "Field `CHN` writer - These bits contain the channel from which the result bits V_VREF were converted."]
pub type ChnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    pub fn v_vref(&self) -> VVrefR {
        VVrefR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the result bits V_VREF were converted."]
    #[inline(always)]
    pub fn chn(&self) -> ChnR {
        ChnR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    #[must_use]
    pub fn v_vref(&mut self) -> VVrefW<GdrSpec> {
        VVrefW::new(self, 6)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the result bits V_VREF were converted."]
    #[inline(always)]
    #[must_use]
    pub fn chn(&mut self) -> ChnW<GdrSpec> {
        ChnW::new(self, 24)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OverrunW<GdrSpec> {
        OverrunW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<GdrSpec> {
        DoneW::new(self, 31)
    }
}
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdrSpec;
impl crate::RegisterSpec for GdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdr::R`](R) reader structure"]
impl crate::Readable for GdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gdr::W`](W) writer structure"]
impl crate::Writable for GdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GdrSpec {
    const RESET_VALUE: u32 = 0;
}
