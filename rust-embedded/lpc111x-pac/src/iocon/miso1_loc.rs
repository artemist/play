#[doc = "Register `MISO1_LOC` reader"]
pub type R = crate::R<Miso1LocSpec>;
#[doc = "Register `MISO1_LOC` writer"]
pub type W = crate::W<Miso1LocSpec>;
#[doc = "Selects pin location for the MISO1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Miso1loc {
    #[doc = "0: `0`"]
    Pio2_2 = 0,
    #[doc = "1: `1`"]
    Pio1_10 = 1,
}
impl From<Miso1loc> for u8 {
    #[inline(always)]
    fn from(variant: Miso1loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Miso1loc {
    type Ux = u8;
}
#[doc = "Field `MISO1LOC` reader - Selects pin location for the MISO1 function."]
pub type Miso1locR = crate::FieldReader<Miso1loc>;
impl Miso1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Miso1loc> {
        match self.bits {
            0 => Some(Miso1loc::Pio2_2),
            1 => Some(Miso1loc::Pio1_10),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == Miso1loc::Pio2_2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio1_10(&self) -> bool {
        *self == Miso1loc::Pio1_10
    }
}
#[doc = "Field `MISO1LOC` writer - Selects pin location for the MISO1 function."]
pub type Miso1locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Miso1loc>;
impl<'a, REG> Miso1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Miso1loc::Pio2_2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Miso1loc::Pio1_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function."]
    #[inline(always)]
    pub fn miso1loc(&self) -> Miso1locR {
        Miso1locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function."]
    #[inline(always)]
    #[must_use]
    pub fn miso1loc(&mut self) -> Miso1locW<Miso1LocSpec> {
        Miso1locW::new(self, 0)
    }
}
#[doc = "MISO1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso1_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso1_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Miso1LocSpec;
impl crate::RegisterSpec for Miso1LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso1_loc::R`](R) reader structure"]
impl crate::Readable for Miso1LocSpec {}
#[doc = "`write(|w| ..)` method takes [`miso1_loc::W`](W) writer structure"]
impl crate::Writable for Miso1LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISO1_LOC to value 0"]
impl crate::Resettable for Miso1LocSpec {
    const RESET_VALUE: u32 = 0;
}
