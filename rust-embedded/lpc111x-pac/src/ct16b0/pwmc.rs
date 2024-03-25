#[doc = "Register `PWMC` reader"]
pub type R = crate::R<PwmcSpec>;
#[doc = "Register `PWMC` writer"]
pub type W = crate::W<PwmcSpec>;
#[doc = "PWM channel0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen0 {
    #[doc = "0: CT16Bn_MAT0 is controlled by EM0."]
    Ct16bnMat0IsContr = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT0."]
    PwmModeIsEnabled_ = 1,
}
impl From<Pwmen0> for bool {
    #[inline(always)]
    fn from(variant: Pwmen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN0` reader - PWM channel0 enable"]
pub type Pwmen0R = crate::BitReader<Pwmen0>;
impl Pwmen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen0 {
        match self.bits {
            false => Pwmen0::Ct16bnMat0IsContr,
            true => Pwmen0::PwmModeIsEnabled_,
        }
    }
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn is_ct16bn_mat0_is_contr(&self) -> bool {
        *self == Pwmen0::Ct16bnMat0IsContr
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == Pwmen0::PwmModeIsEnabled_
    }
}
#[doc = "Field `PWMEN0` writer - PWM channel0 enable"]
pub type Pwmen0W<'a, REG> = crate::BitWriter<'a, REG, Pwmen0>;
impl<'a, REG> Pwmen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn ct16bn_mat0_is_contr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen0::Ct16bnMat0IsContr)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen0::PwmModeIsEnabled_)
    }
}
#[doc = "PWM channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen1 {
    #[doc = "0: CT16Bn_MAT1 is controlled by EM1."]
    Ct16bnMat1IsContr = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT1."]
    PwmModeIsEnabled_ = 1,
}
impl From<Pwmen1> for bool {
    #[inline(always)]
    fn from(variant: Pwmen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN1` reader - PWM channel1 enable"]
pub type Pwmen1R = crate::BitReader<Pwmen1>;
impl Pwmen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen1 {
        match self.bits {
            false => Pwmen1::Ct16bnMat1IsContr,
            true => Pwmen1::PwmModeIsEnabled_,
        }
    }
    #[doc = "CT16Bn_MAT1 is controlled by EM1."]
    #[inline(always)]
    pub fn is_ct16bn_mat1_is_contr(&self) -> bool {
        *self == Pwmen1::Ct16bnMat1IsContr
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == Pwmen1::PwmModeIsEnabled_
    }
}
#[doc = "Field `PWMEN1` writer - PWM channel1 enable"]
pub type Pwmen1W<'a, REG> = crate::BitWriter<'a, REG, Pwmen1>;
impl<'a, REG> Pwmen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CT16Bn_MAT1 is controlled by EM1."]
    #[inline(always)]
    pub fn ct16bn_mat1_is_contr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen1::Ct16bnMat1IsContr)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen1::PwmModeIsEnabled_)
    }
}
#[doc = "PWM channel2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen2 {
    #[doc = "0: Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1."]
    MatchChannel2OrP = 0,
    #[doc = "1: PWM mode is enabled for match channel 2 or pin CT16B0_MAT2."]
    PwmModeIsEnabled_ = 1,
}
impl From<Pwmen2> for bool {
    #[inline(always)]
    fn from(variant: Pwmen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN2` reader - PWM channel2 enable"]
pub type Pwmen2R = crate::BitReader<Pwmen2>;
impl Pwmen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen2 {
        match self.bits {
            false => Pwmen2::MatchChannel2OrP,
            true => Pwmen2::PwmModeIsEnabled_,
        }
    }
    #[doc = "Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1."]
    #[inline(always)]
    pub fn is_match_channel_2_or_p(&self) -> bool {
        *self == Pwmen2::MatchChannel2OrP
    }
    #[doc = "PWM mode is enabled for match channel 2 or pin CT16B0_MAT2."]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == Pwmen2::PwmModeIsEnabled_
    }
}
#[doc = "Field `PWMEN2` writer - PWM channel2 enable"]
pub type Pwmen2W<'a, REG> = crate::BitWriter<'a, REG, Pwmen2>;
impl<'a, REG> Pwmen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1."]
    #[inline(always)]
    pub fn match_channel_2_or_p(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen2::MatchChannel2OrP)
    }
    #[doc = "PWM mode is enabled for match channel 2 or pin CT16B0_MAT2."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen2::PwmModeIsEnabled_)
    }
}
#[doc = "PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen3 {
    #[doc = "0: Match channel 3 match channel 3 is controlled by EM3."]
    MatchChannel3Matc = 0,
    #[doc = "1: PWM mode is enabled for match channel 3match channel 3."]
    PwmModeIsEnabled_ = 1,
}
impl From<Pwmen3> for bool {
    #[inline(always)]
    fn from(variant: Pwmen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN3` reader - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out."]
pub type Pwmen3R = crate::BitReader<Pwmen3>;
impl Pwmen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen3 {
        match self.bits {
            false => Pwmen3::MatchChannel3Matc,
            true => Pwmen3::PwmModeIsEnabled_,
        }
    }
    #[doc = "Match channel 3 match channel 3 is controlled by EM3."]
    #[inline(always)]
    pub fn is_match_channel_3_matc(&self) -> bool {
        *self == Pwmen3::MatchChannel3Matc
    }
    #[doc = "PWM mode is enabled for match channel 3match channel 3."]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == Pwmen3::PwmModeIsEnabled_
    }
}
#[doc = "Field `PWMEN3` writer - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out."]
pub type Pwmen3W<'a, REG> = crate::BitWriter<'a, REG, Pwmen3>;
impl<'a, REG> Pwmen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match channel 3 match channel 3 is controlled by EM3."]
    #[inline(always)]
    pub fn match_channel_3_matc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen3::MatchChannel3Matc)
    }
    #[doc = "PWM mode is enabled for match channel 3match channel 3."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen3::PwmModeIsEnabled_)
    }
}
impl R {
    #[doc = "Bit 0 - PWM channel0 enable"]
    #[inline(always)]
    pub fn pwmen0(&self) -> Pwmen0R {
        Pwmen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM channel1 enable"]
    #[inline(always)]
    pub fn pwmen1(&self) -> Pwmen1R {
        Pwmen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM channel2 enable"]
    #[inline(always)]
    pub fn pwmen2(&self) -> Pwmen2R {
        Pwmen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out."]
    #[inline(always)]
    pub fn pwmen3(&self) -> Pwmen3R {
        Pwmen3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM channel0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwmen0(&mut self) -> Pwmen0W<PwmcSpec> {
        Pwmen0W::new(self, 0)
    }
    #[doc = "Bit 1 - PWM channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwmen1(&mut self) -> Pwmen1W<PwmcSpec> {
        Pwmen1W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM channel2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwmen2(&mut self) -> Pwmen2W<PwmcSpec> {
        Pwmen2W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out."]
    #[inline(always)]
    #[must_use]
    pub fn pwmen3(&mut self) -> Pwmen3W<PwmcSpec> {
        Pwmen3W::new(self, 3)
    }
}
#[doc = "PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT16B0_MAT\\[2:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmcSpec;
impl crate::RegisterSpec for PwmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmc::R`](R) reader structure"]
impl crate::Readable for PwmcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmc::W`](W) writer structure"]
impl crate::Writable for PwmcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMC to value 0"]
impl crate::Resettable for PwmcSpec {
    const RESET_VALUE: u32 = 0;
}
