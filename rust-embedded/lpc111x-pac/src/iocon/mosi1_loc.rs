#[doc = "Register `MOSI1_LOC` reader"]
pub type R = crate::R<Mosi1LocSpec>;
#[doc = "Register `MOSI1_LOC` writer"]
pub type W = crate::W<Mosi1LocSpec>;
#[doc = "Selects pin location for the MOSI1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mosi1loc {
    #[doc = "0: `0`"]
    Pio2_3 = 0,
    #[doc = "1: `1`"]
    Pio1_9 = 1,
}
impl From<Mosi1loc> for u8 {
    #[inline(always)]
    fn from(variant: Mosi1loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mosi1loc {
    type Ux = u8;
}
#[doc = "Field `MOSI1LOC` reader - Selects pin location for the MOSI1 function."]
pub type Mosi1locR = crate::FieldReader<Mosi1loc>;
impl Mosi1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mosi1loc> {
        match self.bits {
            0 => Some(Mosi1loc::Pio2_3),
            1 => Some(Mosi1loc::Pio1_9),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio2_3(&self) -> bool {
        *self == Mosi1loc::Pio2_3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio1_9(&self) -> bool {
        *self == Mosi1loc::Pio1_9
    }
}
#[doc = "Field `MOSI1LOC` writer - Selects pin location for the MOSI1 function."]
pub type Mosi1locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mosi1loc>;
impl<'a, REG> Mosi1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mosi1loc::Pio2_3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mosi1loc::Pio1_9)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the MOSI1 function."]
    #[inline(always)]
    pub fn mosi1loc(&self) -> Mosi1locR {
        Mosi1locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the MOSI1 function."]
    #[inline(always)]
    #[must_use]
    pub fn mosi1loc(&mut self) -> Mosi1locW<Mosi1LocSpec> {
        Mosi1locW::new(self, 0)
    }
}
#[doc = "MOSI1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosi1_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi1_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mosi1LocSpec;
impl crate::RegisterSpec for Mosi1LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mosi1_loc::R`](R) reader structure"]
impl crate::Readable for Mosi1LocSpec {}
#[doc = "`write(|w| ..)` method takes [`mosi1_loc::W`](W) writer structure"]
impl crate::Writable for Mosi1LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOSI1_LOC to value 0"]
impl crate::Resettable for Mosi1LocSpec {
    const RESET_VALUE: u32 = 0;
}
