#[doc = "Register `DCD_LOC` reader"]
pub type R = crate::R<DcdLocSpec>;
#[doc = "Register `DCD_LOC` writer"]
pub type W = crate::W<DcdLocSpec>;
#[doc = "Selects pin location for DCD function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcdloc {
    #[doc = "0: `0`"]
    Pio2_2 = 0,
    #[doc = "1: `1`"]
    Pio3_2 = 1,
}
impl From<Dcdloc> for u8 {
    #[inline(always)]
    fn from(variant: Dcdloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcdloc {
    type Ux = u8;
}
#[doc = "Field `DCDLOC` reader - Selects pin location for DCD function."]
pub type DcdlocR = crate::FieldReader<Dcdloc>;
impl DcdlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dcdloc> {
        match self.bits {
            0 => Some(Dcdloc::Pio2_2),
            1 => Some(Dcdloc::Pio3_2),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == Dcdloc::Pio2_2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_2(&self) -> bool {
        *self == Dcdloc::Pio3_2
    }
}
#[doc = "Field `DCDLOC` writer - Selects pin location for DCD function."]
pub type DcdlocW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dcdloc>;
impl<'a, REG> DcdlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdloc::Pio2_2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdloc::Pio3_2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DCD function."]
    #[inline(always)]
    pub fn dcdloc(&self) -> DcdlocR {
        DcdlocR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DCD function."]
    #[inline(always)]
    #[must_use]
    pub fn dcdloc(&mut self) -> DcdlocW<DcdLocSpec> {
        DcdlocW::new(self, 0)
    }
}
#[doc = "DCD pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcd_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcd_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdLocSpec;
impl crate::RegisterSpec for DcdLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcd_loc::R`](R) reader structure"]
impl crate::Readable for DcdLocSpec {}
#[doc = "`write(|w| ..)` method takes [`dcd_loc::W`](W) writer structure"]
impl crate::Writable for DcdLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCD_LOC to value 0"]
impl crate::Resettable for DcdLocSpec {
    const RESET_VALUE: u32 = 0;
}
