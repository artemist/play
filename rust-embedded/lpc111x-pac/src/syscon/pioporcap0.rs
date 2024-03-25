#[doc = "Register `PIOPORCAP0` reader"]
pub type R = crate::R<Pioporcap0Spec>;
#[doc = "Field `CAPPIO0_n` reader - Raw reset status input PIO0_n: PIO0_11 to PIO0_0"]
pub type Cappio0NR = crate::FieldReader<u16>;
#[doc = "Field `CAPPIO1_n` reader - Raw reset status input PIO1_n: PIO1_11 to PIO1_0"]
pub type Cappio1NR = crate::FieldReader<u16>;
#[doc = "Field `CAPPIO2_n` reader - Raw reset status input PIO2_n: PIO2_7 to PIO2_0"]
pub type Cappio2NR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Raw reset status input PIO0_n: PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_n(&self) -> Cappio0NR {
        Cappio0NR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Raw reset status input PIO1_n: PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_n(&self) -> Cappio1NR {
        Cappio1NR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - Raw reset status input PIO2_n: PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_n(&self) -> Cappio2NR {
        Cappio2NR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "POR captured PIO status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioporcap0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pioporcap0Spec;
impl crate::RegisterSpec for Pioporcap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pioporcap0::R`](R) reader structure"]
impl crate::Readable for Pioporcap0Spec {}
#[doc = "`reset()` method sets PIOPORCAP0 to value 0"]
impl crate::Resettable for Pioporcap0Spec {
    const RESET_VALUE: u32 = 0;
}
