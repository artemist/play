#[doc = "Register `SCK0_LOC` reader"]
pub type R = crate::R<Sck0LocSpec>;
#[doc = "Register `SCK0_LOC` writer"]
pub type W = crate::W<Sck0LocSpec>;
#[doc = "Selects pin location for SCK0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sckloc {
    #[doc = "0: `0`"]
    Pi00_10 = 0,
    #[doc = "1: `1`"]
    Pi02_11 = 1,
    #[doc = "2: `10`"]
    Pi00_6 = 2,
}
impl From<Sckloc> for u8 {
    #[inline(always)]
    fn from(variant: Sckloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sckloc {
    type Ux = u8;
}
#[doc = "Field `SCKLOC` reader - Selects pin location for SCK0 function."]
pub type ScklocR = crate::FieldReader<Sckloc>;
impl ScklocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sckloc> {
        match self.bits {
            0 => Some(Sckloc::Pi00_10),
            1 => Some(Sckloc::Pi02_11),
            2 => Some(Sckloc::Pi00_6),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pi00_10(&self) -> bool {
        *self == Sckloc::Pi00_10
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pi02_11(&self) -> bool {
        *self == Sckloc::Pi02_11
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pi00_6(&self) -> bool {
        *self == Sckloc::Pi00_6
    }
}
#[doc = "Field `SCKLOC` writer - Selects pin location for SCK0 function."]
pub type ScklocW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sckloc>;
impl<'a, REG> ScklocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pi00_10(self) -> &'a mut crate::W<REG> {
        self.variant(Sckloc::Pi00_10)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pi02_11(self) -> &'a mut crate::W<REG> {
        self.variant(Sckloc::Pi02_11)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pi00_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sckloc::Pi00_6)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline(always)]
    pub fn sckloc(&self) -> ScklocR {
        ScklocR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline(always)]
    #[must_use]
    pub fn sckloc(&mut self) -> ScklocW<Sck0LocSpec> {
        ScklocW::new(self, 0)
    }
}
#[doc = "SCK0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sck0_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sck0_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sck0LocSpec;
impl crate::RegisterSpec for Sck0LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sck0_loc::R`](R) reader structure"]
impl crate::Readable for Sck0LocSpec {}
#[doc = "`write(|w| ..)` method takes [`sck0_loc::W`](W) writer structure"]
impl crate::Writable for Sck0LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCK0_LOC to value 0"]
impl crate::Resettable for Sck0LocSpec {
    const RESET_VALUE: u32 = 0;
}
