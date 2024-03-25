#[doc = "Register `PDSLEEPCFG` reader"]
pub type R = crate::R<PdsleepcfgSpec>;
#[doc = "Register `PDSLEEPCFG` writer"]
pub type W = crate::W<PdsleepcfgSpec>;
#[doc = "Field `NOTUSED0` reader - Reserved. Always write these bits as 111."]
pub type Notused0R = crate::FieldReader;
#[doc = "Field `NOTUSED0` writer - Reserved. Always write these bits as 111."]
pub type Notused0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "BOD power-down control in Deep-sleep mode, see Table 40.\n\nValue on reset: 0"]
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
#[doc = "Field `BOD_PD` reader - BOD power-down control in Deep-sleep mode, see Table 40."]
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
#[doc = "Field `BOD_PD` writer - BOD power-down control in Deep-sleep mode, see Table 40."]
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
#[doc = "Field `NOTUSED1` reader - Reserved. Always write these bits as 11."]
pub type Notused1R = crate::FieldReader;
#[doc = "Field `NOTUSED1` writer - Reserved. Always write these bits as 11."]
pub type Notused1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Watchdog oscillator power control in Deep-sleep mode, see Table 40.\n\nValue on reset: 0"]
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
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
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
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
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
#[doc = "Field `NOTUSED2` reader - Reserved. Always write this bit as 1."]
pub type Notused2R = crate::BitReader;
#[doc = "Field `NOTUSED2` writer - Reserved. Always write this bit as 1."]
pub type Notused2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTUSED3` reader - Reserved. Always write these bits as 000."]
pub type Notused3R = crate::FieldReader;
#[doc = "Field `NOTUSED3` writer - Reserved. Always write these bits as 000."]
pub type Notused3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOTUSED4` reader - Reserved. Always write these bits as 11."]
pub type Notused4R = crate::FieldReader;
#[doc = "Field `NOTUSED4` writer - Reserved. Always write these bits as 11."]
pub type Notused4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn notused0(&self) -> Notused0R {
        Notused0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn bod_pd(&self) -> BodPdR {
        BodPdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused1(&self) -> Notused1R {
        Notused1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WdtoscPdR {
        WdtoscPdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused2(&self) -> Notused2R {
        Notused2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline(always)]
    pub fn notused3(&self) -> Notused3R {
        Notused3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused4(&self) -> Notused4R {
        Notused4R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    #[must_use]
    pub fn notused0(&mut self) -> Notused0W<PdsleepcfgSpec> {
        Notused0W::new(self, 0)
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    #[must_use]
    pub fn bod_pd(&mut self) -> BodPdW<PdsleepcfgSpec> {
        BodPdW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    #[must_use]
    pub fn notused1(&mut self) -> Notused1W<PdsleepcfgSpec> {
        Notused1W::new(self, 4)
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    #[must_use]
    pub fn wdtosc_pd(&mut self) -> WdtoscPdW<PdsleepcfgSpec> {
        WdtoscPdW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    #[must_use]
    pub fn notused2(&mut self) -> Notused2W<PdsleepcfgSpec> {
        Notused2W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline(always)]
    #[must_use]
    pub fn notused3(&mut self) -> Notused3W<PdsleepcfgSpec> {
        Notused3W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    #[must_use]
    pub fn notused4(&mut self) -> Notused4W<PdsleepcfgSpec> {
        Notused4W::new(self, 11)
    }
}
#[doc = "Power-down states in Deep-sleep mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdsleepcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdsleepcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsleepcfgSpec;
impl crate::RegisterSpec for PdsleepcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsleepcfg::R`](R) reader structure"]
impl crate::Readable for PdsleepcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pdsleepcfg::W`](W) writer structure"]
impl crate::Writable for PdsleepcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG to value 0"]
impl crate::Resettable for PdsleepcfgSpec {
    const RESET_VALUE: u32 = 0;
}
