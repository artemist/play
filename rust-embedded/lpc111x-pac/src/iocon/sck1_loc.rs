#[doc = "Register `SCK1_LOC` reader"]
pub type R = crate::R<Sck1LocSpec>;
#[doc = "Register `SCK1_LOC` writer"]
pub type W = crate::W<Sck1LocSpec>;
#[doc = "Selects pin location for SCK1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sck1loc {
    #[doc = "0: `0`"]
    Pio2_1 = 0,
    #[doc = "1: `1`"]
    Pio3_2 = 1,
}
impl From<Sck1loc> for u8 {
    #[inline(always)]
    fn from(variant: Sck1loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sck1loc {
    type Ux = u8;
}
#[doc = "Field `SCK1LOC` reader - Selects pin location for SCK1 function."]
pub type Sck1locR = crate::FieldReader<Sck1loc>;
impl Sck1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sck1loc> {
        match self.bits {
            0 => Some(Sck1loc::Pio2_1),
            1 => Some(Sck1loc::Pio3_2),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_1(&self) -> bool {
        *self == Sck1loc::Pio2_1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_2(&self) -> bool {
        *self == Sck1loc::Pio3_2
    }
}
#[doc = "Field `SCK1LOC` writer - Selects pin location for SCK1 function."]
pub type Sck1locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sck1loc>;
impl<'a, REG> Sck1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sck1loc::Pio2_1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sck1loc::Pio3_2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK1 function."]
    #[inline(always)]
    pub fn sck1loc(&self) -> Sck1locR {
        Sck1locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK1 function."]
    #[inline(always)]
    #[must_use]
    pub fn sck1loc(&mut self) -> Sck1locW<Sck1LocSpec> {
        Sck1locW::new(self, 0)
    }
}
#[doc = "SCK1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sck1_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sck1_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sck1LocSpec;
impl crate::RegisterSpec for Sck1LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sck1_loc::R`](R) reader structure"]
impl crate::Readable for Sck1LocSpec {}
#[doc = "`write(|w| ..)` method takes [`sck1_loc::W`](W) writer structure"]
impl crate::Writable for Sck1LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCK1_LOC to value 0"]
impl crate::Resettable for Sck1LocSpec {
    const RESET_VALUE: u32 = 0;
}
