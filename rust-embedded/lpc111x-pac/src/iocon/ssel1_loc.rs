#[doc = "Register `SSEL1_LOC` reader"]
pub type R = crate::R<Ssel1LocSpec>;
#[doc = "Register `SSEL1_LOC` writer"]
pub type W = crate::W<Ssel1LocSpec>;
#[doc = "Selects pin location for SSEL1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssel1loc {
    #[doc = "0: `0`"]
    Pio2_2 = 0,
    #[doc = "1: `1`"]
    Pio3_4 = 1,
}
impl From<Ssel1loc> for u8 {
    #[inline(always)]
    fn from(variant: Ssel1loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssel1loc {
    type Ux = u8;
}
#[doc = "Field `SSEL1LOC` reader - Selects pin location for SSEL1 function."]
pub type Ssel1locR = crate::FieldReader<Ssel1loc>;
impl Ssel1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssel1loc> {
        match self.bits {
            0 => Some(Ssel1loc::Pio2_2),
            1 => Some(Ssel1loc::Pio3_4),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == Ssel1loc::Pio2_2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_4(&self) -> bool {
        *self == Ssel1loc::Pio3_4
    }
}
#[doc = "Field `SSEL1LOC` writer - Selects pin location for SSEL1 function."]
pub type Ssel1locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ssel1loc>;
impl<'a, REG> Ssel1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel1loc::Pio2_2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel1loc::Pio3_4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline(always)]
    pub fn ssel1loc(&self) -> Ssel1locR {
        Ssel1locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline(always)]
    #[must_use]
    pub fn ssel1loc(&mut self) -> Ssel1locW<Ssel1LocSpec> {
        Ssel1locW::new(self, 0)
    }
}
#[doc = "SSEL1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssel1_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssel1_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssel1LocSpec;
impl crate::RegisterSpec for Ssel1LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssel1_loc::R`](R) reader structure"]
impl crate::Readable for Ssel1LocSpec {}
#[doc = "`write(|w| ..)` method takes [`ssel1_loc::W`](W) writer structure"]
impl crate::Writable for Ssel1LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSEL1_LOC to value 0"]
impl crate::Resettable for Ssel1LocSpec {
    const RESET_VALUE: u32 = 0;
}
