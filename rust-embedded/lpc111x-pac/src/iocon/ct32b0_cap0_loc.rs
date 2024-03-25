#[doc = "Register `CT32B0_CAP0_LOC` reader"]
pub type R = crate::R<Ct32b0Cap0LocSpec>;
#[doc = "Register `CT32B0_CAP0_LOC` writer"]
pub type W = crate::W<Ct32b0Cap0LocSpec>;
#[doc = "Selects pin location for the CT32B0_CAP0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ct32b0Cap0loc {
    #[doc = "0: `0`"]
    Pio1_5 = 0,
    #[doc = "1: `1`"]
    Pio2_9 = 1,
}
impl From<Ct32b0Cap0loc> for u8 {
    #[inline(always)]
    fn from(variant: Ct32b0Cap0loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ct32b0Cap0loc {
    type Ux = u8;
}
#[doc = "Field `CT32B0_CAP0LOC` reader - Selects pin location for the CT32B0_CAP0 function."]
pub type Ct32b0Cap0locR = crate::FieldReader<Ct32b0Cap0loc>;
impl Ct32b0Cap0locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ct32b0Cap0loc> {
        match self.bits {
            0 => Some(Ct32b0Cap0loc::Pio1_5),
            1 => Some(Ct32b0Cap0loc::Pio2_9),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio1_5(&self) -> bool {
        *self == Ct32b0Cap0loc::Pio1_5
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio2_9(&self) -> bool {
        *self == Ct32b0Cap0loc::Pio2_9
    }
}
#[doc = "Field `CT32B0_CAP0LOC` writer - Selects pin location for the CT32B0_CAP0 function."]
pub type Ct32b0Cap0locW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ct32b0Cap0loc>;
impl<'a, REG> Ct32b0Cap0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b0Cap0loc::Pio1_5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio2_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32b0Cap0loc::Pio2_9)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline(always)]
    pub fn ct32b0_cap0loc(&self) -> Ct32b0Cap0locR {
        Ct32b0Cap0locR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline(always)]
    #[must_use]
    pub fn ct32b0_cap0loc(&mut self) -> Ct32b0Cap0locW<Ct32b0Cap0LocSpec> {
        Ct32b0Cap0locW::new(self, 0)
    }
}
#[doc = "CT32B0_CAP0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct32b0_cap0_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct32b0_cap0_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ct32b0Cap0LocSpec;
impl crate::RegisterSpec for Ct32b0Cap0LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct32b0_cap0_loc::R`](R) reader structure"]
impl crate::Readable for Ct32b0Cap0LocSpec {}
#[doc = "`write(|w| ..)` method takes [`ct32b0_cap0_loc::W`](W) writer structure"]
impl crate::Writable for Ct32b0Cap0LocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT32B0_CAP0_LOC to value 0"]
impl crate::Resettable for Ct32b0Cap0LocSpec {
    const RESET_VALUE: u32 = 0;
}
