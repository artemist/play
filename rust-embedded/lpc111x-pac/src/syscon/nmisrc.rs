#[doc = "Register `NMISRC` reader"]
pub type R = crate::R<NmisrcSpec>;
#[doc = "Register `NMISRC` writer"]
pub type W = crate::W<NmisrcSpec>;
#[doc = "Field `IRQNO` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 in this register is 1. See Table 54 for the list of interrupt sources and their IRQ numbers."]
pub type IrqnoR = crate::FieldReader;
#[doc = "Field `IRQNO` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 in this register is 1. See Table 54 for the list of interrupt sources and their IRQ numbers."]
pub type IrqnoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NMIEN` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub type NmienR = crate::BitReader;
#[doc = "Field `NMIEN` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub type NmienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 in this register is 1. See Table 54 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    pub fn irqno(&self) -> IrqnoR {
        IrqnoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&self) -> NmienR {
        NmienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 in this register is 1. See Table 54 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    #[must_use]
    pub fn irqno(&mut self) -> IrqnoW<NmisrcSpec> {
        IrqnoW::new(self, 0)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NmienW<NmisrcSpec> {
        NmienW::new(self, 31)
    }
}
#[doc = "NMI source selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmisrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmisrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmisrcSpec;
impl crate::RegisterSpec for NmisrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmisrc::R`](R) reader structure"]
impl crate::Readable for NmisrcSpec {}
#[doc = "`write(|w| ..)` method takes [`nmisrc::W`](W) writer structure"]
impl crate::Writable for NmisrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NmisrcSpec {
    const RESET_VALUE: u32 = 0;
}
