#[doc = "Register `BODCTRL` reader"]
pub type R = crate::R<BodctrlSpec>;
#[doc = "Register `BODCTRL` writer"]
pub type W = crate::W<BodctrlSpec>;
#[doc = "BOD reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bodrstlev {
    #[doc = "0: Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    Level0TheResetA = 0,
    #[doc = "1: Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    Level1TheResetA = 1,
    #[doc = "2: Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    Level2TheResetA = 2,
    #[doc = "3: Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    Level3TheResetA = 3,
}
impl From<Bodrstlev> for u8 {
    #[inline(always)]
    fn from(variant: Bodrstlev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bodrstlev {
    type Ux = u8;
}
#[doc = "Field `BODRSTLEV` reader - BOD reset level"]
pub type BodrstlevR = crate::FieldReader<Bodrstlev>;
impl BodrstlevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrstlev {
        match self.bits {
            0 => Bodrstlev::Level0TheResetA,
            1 => Bodrstlev::Level1TheResetA,
            2 => Bodrstlev::Level2TheResetA,
            3 => Bodrstlev::Level3TheResetA,
            _ => unreachable!(),
        }
    }
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    #[inline(always)]
    pub fn is_level_0_the_reset_a(&self) -> bool {
        *self == Bodrstlev::Level0TheResetA
    }
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    #[inline(always)]
    pub fn is_level_1_the_reset_a(&self) -> bool {
        *self == Bodrstlev::Level1TheResetA
    }
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    #[inline(always)]
    pub fn is_level_2_the_reset_a(&self) -> bool {
        *self == Bodrstlev::Level2TheResetA
    }
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    #[inline(always)]
    pub fn is_level_3_the_reset_a(&self) -> bool {
        *self == Bodrstlev::Level3TheResetA
    }
}
#[doc = "Field `BODRSTLEV` writer - BOD reset level"]
pub type BodrstlevW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Bodrstlev>;
impl<'a, REG> BodrstlevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    #[inline(always)]
    pub fn level_0_the_reset_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstlev::Level0TheResetA)
    }
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    #[inline(always)]
    pub fn level_1_the_reset_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstlev::Level1TheResetA)
    }
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    #[inline(always)]
    pub fn level_2_the_reset_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstlev::Level2TheResetA)
    }
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    #[inline(always)]
    pub fn level_3_the_reset_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstlev::Level3TheResetA)
    }
}
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bodintval {
    #[doc = "0: Level 0: The interrupt assertion threshold voltage is 1.65 V; the interrupt de-assertion threshold voltage is 1.80 V."]
    Level0TheInterru = 0,
    #[doc = "1: Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    Level1theInterrup = 1,
    #[doc = "2: Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    Level2TheInterru = 2,
    #[doc = "3: Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    Level3TheInterru = 3,
}
impl From<Bodintval> for u8 {
    #[inline(always)]
    fn from(variant: Bodintval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bodintval {
    type Ux = u8;
}
#[doc = "Field `BODINTVAL` reader - BOD interrupt level"]
pub type BodintvalR = crate::FieldReader<Bodintval>;
impl BodintvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodintval {
        match self.bits {
            0 => Bodintval::Level0TheInterru,
            1 => Bodintval::Level1theInterrup,
            2 => Bodintval::Level2TheInterru,
            3 => Bodintval::Level3TheInterru,
            _ => unreachable!(),
        }
    }
    #[doc = "Level 0: The interrupt assertion threshold voltage is 1.65 V; the interrupt de-assertion threshold voltage is 1.80 V."]
    #[inline(always)]
    pub fn is_level_0_the_interru(&self) -> bool {
        *self == Bodintval::Level0TheInterru
    }
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    #[inline(always)]
    pub fn is_level_1the_interrup(&self) -> bool {
        *self == Bodintval::Level1theInterrup
    }
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    #[inline(always)]
    pub fn is_level_2_the_interru(&self) -> bool {
        *self == Bodintval::Level2TheInterru
    }
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    #[inline(always)]
    pub fn is_level_3_the_interru(&self) -> bool {
        *self == Bodintval::Level3TheInterru
    }
}
#[doc = "Field `BODINTVAL` writer - BOD interrupt level"]
pub type BodintvalW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Bodintval>;
impl<'a, REG> BodintvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level 0: The interrupt assertion threshold voltage is 1.65 V; the interrupt de-assertion threshold voltage is 1.80 V."]
    #[inline(always)]
    pub fn level_0_the_interru(self) -> &'a mut crate::W<REG> {
        self.variant(Bodintval::Level0TheInterru)
    }
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    #[inline(always)]
    pub fn level_1the_interrup(self) -> &'a mut crate::W<REG> {
        self.variant(Bodintval::Level1theInterrup)
    }
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    #[inline(always)]
    pub fn level_2_the_interru(self) -> &'a mut crate::W<REG> {
        self.variant(Bodintval::Level2TheInterru)
    }
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    #[inline(always)]
    pub fn level_3_the_interru(self) -> &'a mut crate::W<REG> {
        self.variant(Bodintval::Level3TheInterru)
    }
}
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrstena {
    #[doc = "0: Disable reset function."]
    DisableResetFuncti = 0,
    #[doc = "1: Enable reset function."]
    EnableResetFunctio = 1,
}
impl From<Bodrstena> for bool {
    #[inline(always)]
    fn from(variant: Bodrstena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTENA` reader - BOD reset enable"]
pub type BodrstenaR = crate::BitReader<Bodrstena>;
impl BodrstenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrstena {
        match self.bits {
            false => Bodrstena::DisableResetFuncti,
            true => Bodrstena::EnableResetFunctio,
        }
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == Bodrstena::DisableResetFuncti
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == Bodrstena::EnableResetFunctio
    }
}
#[doc = "Field `BODRSTENA` writer - BOD reset enable"]
pub type BodrstenaW<'a, REG> = crate::BitWriter<'a, REG, Bodrstena>;
impl<'a, REG> BodrstenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable_reset_functi(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstena::DisableResetFuncti)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable_reset_functio(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstena::EnableResetFunctio)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BodrstlevR {
        BodrstlevR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&self) -> BodintvalR {
        BodintvalR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BodrstenaR {
        BodrstenaR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    #[must_use]
    pub fn bodrstlev(&mut self) -> BodrstlevW<BodctrlSpec> {
        BodrstlevW::new(self, 0)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    #[must_use]
    pub fn bodintval(&mut self) -> BodintvalW<BodctrlSpec> {
        BodintvalW::new(self, 2)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrstena(&mut self) -> BodrstenaW<BodctrlSpec> {
        BodrstenaW::new(self, 4)
    }
}
#[doc = "BOD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodctrlSpec;
impl crate::RegisterSpec for BodctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodctrl::R`](R) reader structure"]
impl crate::Readable for BodctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bodctrl::W`](W) writer structure"]
impl crate::Writable for BodctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BodctrlSpec {
    const RESET_VALUE: u32 = 0;
}
