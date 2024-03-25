#[doc = "Register `RI_LOC` reader"]
pub type R = crate::R<RiLocSpec>;
#[doc = "Register `RI_LOC` writer"]
pub type W = crate::W<RiLocSpec>;
#[doc = "Selects pin location for RI function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Riloc {
    #[doc = "0: `0`"]
    Pio2_3 = 0,
    #[doc = "1: `1`"]
    Pio3_3 = 1,
}
impl From<Riloc> for u8 {
    #[inline(always)]
    fn from(variant: Riloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Riloc {
    type Ux = u8;
}
#[doc = "Field `RILOC` reader - Selects pin location for RI function."]
pub type RilocR = crate::FieldReader<Riloc>;
impl RilocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Riloc> {
        match self.bits {
            0 => Some(Riloc::Pio2_3),
            1 => Some(Riloc::Pio3_3),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_3(&self) -> bool {
        *self == Riloc::Pio2_3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_3(&self) -> bool {
        *self == Riloc::Pio3_3
    }
}
#[doc = "Field `RILOC` writer - Selects pin location for RI function."]
pub type RilocW<'a, REG> = crate::FieldWriter<'a, REG, 2, Riloc>;
impl<'a, REG> RilocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Riloc::Pio2_3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_3(self) -> &'a mut crate::W<REG> {
        self.variant(Riloc::Pio3_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for RI function."]
    #[inline(always)]
    pub fn riloc(&self) -> RilocR {
        RilocR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for RI function."]
    #[inline(always)]
    #[must_use]
    pub fn riloc(&mut self) -> RilocW<RiLocSpec> {
        RilocW::new(self, 0)
    }
}
#[doc = "RI pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ri_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ri_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RiLocSpec;
impl crate::RegisterSpec for RiLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ri_loc::R`](R) reader structure"]
impl crate::Readable for RiLocSpec {}
#[doc = "`write(|w| ..)` method takes [`ri_loc::W`](W) writer structure"]
impl crate::Writable for RiLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RI_LOC to value 0"]
impl crate::Resettable for RiLocSpec {
    const RESET_VALUE: u32 = 0;
}
