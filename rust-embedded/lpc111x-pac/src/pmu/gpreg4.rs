#[doc = "Register `GPREG4` reader"]
pub type R = crate::R<Gpreg4Spec>;
#[doc = "Register `GPREG4` writer"]
pub type W = crate::W<Gpreg4Spec>;
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeuphys {
    #[doc = "1: Hysteresis for WAKEUP pin enabled."]
    Enabled = 1,
    #[doc = "0: Hysteresis for WAKUP pin disabled."]
    Disabled = 0,
}
impl From<Wakeuphys> for bool {
    #[inline(always)]
    fn from(variant: Wakeuphys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUPHYS` reader - WAKEUP pin hysteresis enable"]
pub type WakeuphysR = crate::BitReader<Wakeuphys>;
impl WakeuphysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeuphys {
        match self.bits {
            true => Wakeuphys::Enabled,
            false => Wakeuphys::Disabled,
        }
    }
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wakeuphys::Enabled
    }
    #[doc = "Hysteresis for WAKUP pin disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wakeuphys::Disabled
    }
}
#[doc = "Field `WAKEUPHYS` writer - WAKEUP pin hysteresis enable"]
pub type WakeuphysW<'a, REG> = crate::BitWriter<'a, REG, Wakeuphys>;
impl<'a, REG> WakeuphysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeuphys::Enabled)
    }
    #[doc = "Hysteresis for WAKUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeuphys::Disabled)
    }
}
#[doc = "Field `GPDATA` reader - Data retained during Deep power-down mode."]
pub type GpdataR = crate::FieldReader<u32>;
#[doc = "Field `GPDATA` writer - Data retained during Deep power-down mode."]
pub type GpdataW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WakeuphysR {
        WakeuphysR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GpdataR {
        GpdataR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeuphys(&mut self) -> WakeuphysW<Gpreg4Spec> {
        WakeuphysW::new(self, 10)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpdata(&mut self) -> GpdataW<Gpreg4Spec> {
        GpdataW::new(self, 11)
    }
}
#[doc = "General purpose register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpreg4Spec;
impl crate::RegisterSpec for Gpreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpreg4::R`](R) reader structure"]
impl crate::Readable for Gpreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpreg4::W`](W) writer structure"]
impl crate::Writable for Gpreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPREG4 to value 0"]
impl crate::Resettable for Gpreg4Spec {
    const RESET_VALUE: u32 = 0;
}
