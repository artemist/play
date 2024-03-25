#[doc = "Register `SYSPLLCTRL` reader"]
pub type R = crate::R<SyspllctrlSpec>;
#[doc = "Register `SYSPLLCTRL` writer"]
pub type W = crate::W<SyspllctrlSpec>;
#[doc = "Field `MSEL` reader - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32."]
pub type MselR = crate::FieldReader;
#[doc = "Field `MSEL` writer - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32."]
pub type MselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Post divider ratio P. The division ratio is 2 x P.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: P = 1"]
    PEq1 = 0,
    #[doc = "1: P = 2"]
    PEq2 = 1,
    #[doc = "2: P = 4"]
    PEq4 = 2,
    #[doc = "3: P = 8"]
    PEq8 = 3,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
#[doc = "Field `PSEL` reader - Post divider ratio P. The division ratio is 2 x P."]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psel {
        match self.bits {
            0 => Psel::PEq1,
            1 => Psel::PEq2,
            2 => Psel::PEq4,
            3 => Psel::PEq8,
            _ => unreachable!(),
        }
    }
    #[doc = "P = 1"]
    #[inline(always)]
    pub fn is_p_eq_1(&self) -> bool {
        *self == Psel::PEq1
    }
    #[doc = "P = 2"]
    #[inline(always)]
    pub fn is_p_eq_2(&self) -> bool {
        *self == Psel::PEq2
    }
    #[doc = "P = 4"]
    #[inline(always)]
    pub fn is_p_eq_4(&self) -> bool {
        *self == Psel::PEq4
    }
    #[doc = "P = 8"]
    #[inline(always)]
    pub fn is_p_eq_8(&self) -> bool {
        *self == Psel::PEq8
    }
}
#[doc = "Field `PSEL` writer - Post divider ratio P. The division ratio is 2 x P."]
pub type PselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Psel>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "P = 1"]
    #[inline(always)]
    pub fn p_eq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::PEq1)
    }
    #[doc = "P = 2"]
    #[inline(always)]
    pub fn p_eq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::PEq2)
    }
    #[doc = "P = 4"]
    #[inline(always)]
    pub fn p_eq_4(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::PEq4)
    }
    #[doc = "P = 8"]
    #[inline(always)]
    pub fn p_eq_8(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::PEq8)
    }
}
impl R {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32."]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32."]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MselW<SyspllctrlSpec> {
        MselW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<SyspllctrlSpec> {
        PselW::new(self, 5)
    }
}
#[doc = "System PLL control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syspllctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllctrlSpec;
impl crate::RegisterSpec for SyspllctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllctrl::R`](R) reader structure"]
impl crate::Readable for SyspllctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`syspllctrl::W`](W) writer structure"]
impl crate::Writable for SyspllctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLCTRL to value 0"]
impl crate::Resettable for SyspllctrlSpec {
    const RESET_VALUE: u32 = 0;
}
