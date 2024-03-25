#[doc = "Register `ADR%s` reader"]
pub type R = crate::R<AdrSpec>;
#[doc = "Register `ADR%s` writer"]
pub type W = crate::W<AdrSpec>;
#[doc = "Field `GC` reader - General Call enable bit."]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - General Call enable bit."]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Address` reader - The I2C device address for slave mode."]
pub type AddressR = crate::FieldReader;
#[doc = "Field `Address` writer - The I2C device address for slave mode."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GcW<AdrSpec> {
        GcW::new(self, 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<AdrSpec> {
        AddressW::new(self, 1)
    }
}
#[doc = "I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdrSpec;
impl crate::RegisterSpec for AdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for AdrSpec {}
#[doc = "`write(|w| ..)` method takes [`adr::W`](W) writer structure"]
impl crate::Writable for AdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADR%s to value 0"]
impl crate::Resettable for AdrSpec {
    const RESET_VALUE: u32 = 0;
}
