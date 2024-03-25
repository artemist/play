#[doc = "Register `CT16B0_CAP0_LOC` reader"]
pub type R = crate::R<Ct16b0Cap0LocSpec>;
#[doc = "Register `CT16B0_CAP0_LOC` writer"]
pub type W = crate::W<Ct16b0Cap0LocSpec>;
#[doc = "Selects pin location for CT16B0_CAP0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ct16b0Cap0loc {
    #[doc = "0: `0`"]
    Pio0_2 = 0,
    #[doc = "1: `1`"]
    Pio3_3 = 1,
}
impl From<Ct16b0Cap0loc> for u8 {
    #[inline(always)]
    fn from(variant: Ct16b0Cap0loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ct16b0Cap0loc {
    type Ux = u8;
}
#[doc = "Field `CT16B0_CAP0LOC` reader - Selects pin location for CT16B0_CAP0 function."]
pub type Ct16b0Cap0locR = crate::FieldReader<Ct16b0Cap0loc>;
impl Ct16b0Cap0locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ct16b0Cap0loc> {
        match self.bits {
            0 => Some(Ct16b0Cap0loc::Pio0_2),
            1 => Some(Ct16b0Cap0loc::Pio3_3),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio0_2(&self) -> bool {
        *self == Ct16b0Cap0loc::Pio0_2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio3_3(&self) -> bool {
        *self == Ct16b0Cap0loc::Pio3_3
    }
}
#[doc = "Field `CT16B0_CAP0LOC` writer - Selects pin location for CT16B0_CAP0 function."]
pub type Ct16b0Cap0locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ct16b0Cap0loc>;
impl<'a, REG> Ct16b0Cap0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b0Cap0loc::Pio0_2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio3_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ct16b0Cap0loc::Pio3_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function."]
    #[inline(always)]
    pub fn ct16b0_cap0loc(&self) -> Ct16b0Cap0locR {
        Ct16b0Cap0locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function."]
    #[inline(always)]
    #[must_use]
    pub fn ct16b0_cap0loc(&mut self) -> Ct16b0Cap0locW<Ct16b0Cap0LocSpec> {
        Ct16b0Cap0locW::new(self, 0)
    }
}
#[doc = "CT16B0_CAP0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct16b0_cap0_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct16b0_cap0_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ct16b0Cap0LocSpec;
impl crate::RegisterSpec for Ct16b0Cap0LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct16b0_cap0_loc::R`](R) reader structure"]
impl crate::Readable for Ct16b0Cap0LocSpec {}
#[doc = "`write(|w| ..)` method takes [`ct16b0_cap0_loc::W`](W) writer structure"]
impl crate::Writable for Ct16b0Cap0LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT16B0_CAP0_LOC to value 0"]
impl crate::Resettable for Ct16b0Cap0LocSpec {
    const RESET_VALUE: u32 = 0;
}
