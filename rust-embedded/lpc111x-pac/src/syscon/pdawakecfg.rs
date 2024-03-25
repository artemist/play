#[doc = "Register `PDAWAKECFG` reader"]
pub type R = crate::R<PdawakecfgSpec>;
#[doc = "Register `PDAWAKECFG` writer"]
pub type W = crate::W<PdawakecfgSpec>;
#[doc = "IRC oscillator output wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrcoutPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<IrcoutPd> for bool {
    #[inline(always)]
    fn from(variant: IrcoutPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCOUT_PD` reader - IRC oscillator output wake-up configuration"]
pub type IrcoutPdR = crate::BitReader<IrcoutPd>;
impl IrcoutPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrcoutPd {
        match self.bits {
            false => IrcoutPd::Powered,
            true => IrcoutPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IrcoutPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IrcoutPd::PoweredDown
    }
}
#[doc = "Field `IRCOUT_PD` writer - IRC oscillator output wake-up configuration"]
pub type IrcoutPdW<'a, REG> = crate::BitWriter<'a, REG, IrcoutPd>;
impl<'a, REG> IrcoutPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(IrcoutPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(IrcoutPd::PoweredDown)
    }
}
#[doc = "IRC oscillator power-down wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrcPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<IrcPd> for bool {
    #[inline(always)]
    fn from(variant: IrcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC_PD` reader - IRC oscillator power-down wake-up configuration"]
pub type IrcPdR = crate::BitReader<IrcPd>;
impl IrcPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrcPd {
        match self.bits {
            false => IrcPd::Powered,
            true => IrcPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IrcPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IrcPd::PoweredDown
    }
}
#[doc = "Field `IRC_PD` writer - IRC oscillator power-down wake-up configuration"]
pub type IrcPdW<'a, REG> = crate::BitWriter<'a, REG, IrcPd>;
impl<'a, REG> IrcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(IrcPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(IrcPd::PoweredDown)
    }
}
#[doc = "Flash wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<FlashPd> for bool {
    #[inline(always)]
    fn from(variant: FlashPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_PD` reader - Flash wake-up configuration"]
pub type FlashPdR = crate::BitReader<FlashPd>;
impl FlashPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashPd {
        match self.bits {
            false => FlashPd::Powered,
            true => FlashPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == FlashPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == FlashPd::PoweredDown
    }
}
#[doc = "Field `FLASH_PD` writer - Flash wake-up configuration"]
pub type FlashPdW<'a, REG> = crate::BitWriter<'a, REG, FlashPd>;
impl<'a, REG> FlashPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(FlashPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(FlashPd::PoweredDown)
    }
}
#[doc = "BOD wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<BodPd> for bool {
    #[inline(always)]
    fn from(variant: BodPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD_PD` reader - BOD wake-up configuration"]
pub type BodPdR = crate::BitReader<BodPd>;
impl BodPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodPd {
        match self.bits {
            false => BodPd::Powered,
            true => BodPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BodPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BodPd::PoweredDown
    }
}
#[doc = "Field `BOD_PD` writer - BOD wake-up configuration"]
pub type BodPdW<'a, REG> = crate::BitWriter<'a, REG, BodPd>;
impl<'a, REG> BodPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(BodPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(BodPd::PoweredDown)
    }
}
#[doc = "ADC wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<AdcPd> for bool {
    #[inline(always)]
    fn from(variant: AdcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` reader - ADC wake-up configuration"]
pub type AdcPdR = crate::BitReader<AdcPd>;
impl AdcPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcPd {
        match self.bits {
            false => AdcPd::Powered,
            true => AdcPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == AdcPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == AdcPd::PoweredDown
    }
}
#[doc = "Field `ADC_PD` writer - ADC wake-up configuration"]
pub type AdcPdW<'a, REG> = crate::BitWriter<'a, REG, AdcPd>;
impl<'a, REG> AdcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::PoweredDown)
    }
}
#[doc = "System oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysoscPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<SysoscPd> for bool {
    #[inline(always)]
    fn from(variant: SysoscPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSC_PD` reader - System oscillator wake-up configuration"]
pub type SysoscPdR = crate::BitReader<SysoscPd>;
impl SysoscPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysoscPd {
        match self.bits {
            false => SysoscPd::Powered,
            true => SysoscPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SysoscPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SysoscPd::PoweredDown
    }
}
#[doc = "Field `SYSOSC_PD` writer - System oscillator wake-up configuration"]
pub type SysoscPdW<'a, REG> = crate::BitWriter<'a, REG, SysoscPd>;
impl<'a, REG> SysoscPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(SysoscPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(SysoscPd::PoweredDown)
    }
}
#[doc = "Watchdog oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtoscPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<WdtoscPd> for bool {
    #[inline(always)]
    fn from(variant: WdtoscPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator wake-up configuration"]
pub type WdtoscPdR = crate::BitReader<WdtoscPd>;
impl WdtoscPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtoscPd {
        match self.bits {
            false => WdtoscPd::Powered,
            true => WdtoscPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WdtoscPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WdtoscPd::PoweredDown
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator wake-up configuration"]
pub type WdtoscPdW<'a, REG> = crate::BitWriter<'a, REG, WdtoscPd>;
impl<'a, REG> WdtoscPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(WdtoscPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(WdtoscPd::PoweredDown)
    }
}
#[doc = "System PLL wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllPd {
    #[doc = "0: Powered"]
    Powered = 0,
    #[doc = "1: Powered down"]
    PoweredDown = 1,
}
impl From<SyspllPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLL_PD` reader - System PLL wake-up configuration"]
pub type SyspllPdR = crate::BitReader<SyspllPd>;
impl SyspllPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyspllPd {
        match self.bits {
            false => SyspllPd::Powered,
            true => SyspllPd::PoweredDown,
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SyspllPd::Powered
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SyspllPd::PoweredDown
    }
}
#[doc = "Field `SYSPLL_PD` writer - System PLL wake-up configuration"]
pub type SyspllPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllPd>;
impl<'a, REG> SyspllPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllPd::Powered)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllPd::PoweredDown)
    }
}
#[doc = "Field `NOTUSED0` reader - Reserved. Always write this bit as 1."]
pub type Notused0R = crate::BitReader;
#[doc = "Field `NOTUSED0` writer - Reserved. Always write this bit as 1."]
pub type Notused0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED1` reader - Reserved. Always write this bit as 0."]
pub type Notused1R = crate::BitReader;
#[doc = "Field `NOTUSED1` writer - Reserved. Always write this bit as 0."]
pub type Notused1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED2` reader - Reserved. Always write this bit as 1."]
pub type Notused2R = crate::BitReader;
#[doc = "Field `NOTUSED2` writer - Reserved. Always write this bit as 1."]
pub type Notused2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED3` reader - Reserved. Always write this bit as 1."]
pub type Notused3R = crate::BitReader;
#[doc = "Field `NOTUSED3` writer - Reserved. Always write this bit as 1."]
pub type Notused3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED4` reader - Reserved. Always write this bit as 0."]
pub type Notused4R = crate::BitReader;
#[doc = "Field `NOTUSED4` writer - Reserved. Always write this bit as 0."]
pub type Notused4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED5` reader - Reserved. Always write these bits as 111."]
pub type Notused5R = crate::FieldReader;
#[doc = "Field `NOTUSED5` writer - Reserved. Always write these bits as 111."]
pub type Notused5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&self) -> IrcoutPdR {
        IrcoutPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&self) -> IrcPdR {
        IrcPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FlashPdR {
        FlashPdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BodPdR {
        BodPdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&self) -> AdcPdR {
        AdcPdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SysoscPdR {
        SysoscPdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WdtoscPdR {
        WdtoscPdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SyspllPdR {
        SyspllPdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused0(&self) -> Notused0R {
        Notused0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    pub fn notused1(&self) -> Notused1R {
        Notused1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused2(&self) -> Notused2R {
        Notused2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused3(&self) -> Notused3R {
        Notused3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    pub fn notused4(&self) -> Notused4R {
        Notused4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn notused5(&self) -> Notused5R {
        Notused5R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ircout_pd(&mut self) -> IrcoutPdW<PdawakecfgSpec> {
        IrcoutPdW::new(self, 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn irc_pd(&mut self) -> IrcPdW<PdawakecfgSpec> {
        IrcPdW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flash_pd(&mut self) -> FlashPdW<PdawakecfgSpec> {
        FlashPdW::new(self, 2)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bod_pd(&mut self) -> BodPdW<PdawakecfgSpec> {
        BodPdW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> AdcPdW<PdawakecfgSpec> {
        AdcPdW::new(self, 4)
    }
    #[doc = "Bit 5 - System oscillator wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sysosc_pd(&mut self) -> SysoscPdW<PdawakecfgSpec> {
        SysoscPdW::new(self, 5)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdtosc_pd(&mut self) -> WdtoscPdW<PdawakecfgSpec> {
        WdtoscPdW::new(self, 6)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    #[must_use]
    pub fn syspll_pd(&mut self) -> SyspllPdW<PdawakecfgSpec> {
        SyspllPdW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    #[must_use]
    pub fn notused0(&mut self) -> Notused0W<PdawakecfgSpec> {
        Notused0W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    #[must_use]
    pub fn notused1(&mut self) -> Notused1W<PdawakecfgSpec> {
        Notused1W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    #[must_use]
    pub fn notused2(&mut self) -> Notused2W<PdawakecfgSpec> {
        Notused2W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    #[must_use]
    pub fn notused3(&mut self) -> Notused3W<PdawakecfgSpec> {
        Notused3W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    #[must_use]
    pub fn notused4(&mut self) -> Notused4W<PdawakecfgSpec> {
        Notused4W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    #[must_use]
    pub fn notused5(&mut self) -> Notused5W<PdawakecfgSpec> {
        Notused5W::new(self, 13)
    }
}
#[doc = "Power-down states after wake-up from Deep-sleep mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdawakecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdawakecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdawakecfgSpec;
impl crate::RegisterSpec for PdawakecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdawakecfg::R`](R) reader structure"]
impl crate::Readable for PdawakecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pdawakecfg::W`](W) writer structure"]
impl crate::Writable for PdawakecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDAWAKECFG to value 0xedf0"]
impl crate::Resettable for PdawakecfgSpec {
    const RESET_VALUE: u32 = 0xedf0;
}
