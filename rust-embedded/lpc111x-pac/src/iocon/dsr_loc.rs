#[doc = "Register `DSR_LOC` reader"]
pub type R = crate::R<DsrLocSpec>;
#[doc = "Register `DSR_LOC` writer"]
pub type W = crate::W<DsrLocSpec>;
#[doc = "Selects pin location for DSR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsrloc {
    #[doc = "0: `0`"]
    Pio2_1 = 0,
    #[doc = "1: `1`"]
    Pio3_1 = 1,
}
impl From<Dsrloc> for u8 {
    #[inline(always)]
    fn from(variant: Dsrloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsrloc {
    type Ux = u8;
}
#[doc = "Field `DSRLOC` reader - Selects pin location for DSR function."]
pub type DsrlocR = crate::FieldReader<Dsrloc>;
impl DsrlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsrloc> {
        match self.bits {
            0 => Some(Dsrloc::Pio2_1),
            1 => Some(Dsrloc::Pio3_1),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_1(&self) -> bool {
        *self == Dsrloc::Pio2_1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_1(&self) -> bool {
        *self == Dsrloc::Pio3_1
    }
}
#[doc = "Field `DSRLOC` writer - Selects pin location for DSR function."]
pub type DsrlocW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsrloc>;
impl<'a, REG> DsrlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrloc::Pio2_1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrloc::Pio3_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline(always)]
    pub fn dsrloc(&self) -> DsrlocR {
        DsrlocR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline(always)]
    #[must_use]
    pub fn dsrloc(&mut self) -> DsrlocW<DsrLocSpec> {
        DsrlocW::new(self, 0)
    }
}
#[doc = "DSR pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsr_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrLocSpec;
impl crate::RegisterSpec for DsrLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr_loc::R`](R) reader structure"]
impl crate::Readable for DsrLocSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr_loc::W`](W) writer structure"]
impl crate::Writable for DsrLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSR_LOC to value 0"]
impl crate::Resettable for DsrLocSpec {
    const RESET_VALUE: u32 = 0;
}
