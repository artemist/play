#[doc = "Register `PCON` reader"]
pub type R = crate::R<PconSpec>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PconSpec>;
#[doc = "Deep power-down mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpden {
    #[doc = "0: ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)."]
    Sleepmode = 0,
    #[doc = "1: ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    Deeppowerdown = 1,
}
impl From<Dpden> for bool {
    #[inline(always)]
    fn from(variant: Dpden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDEN` reader - Deep power-down mode enable"]
pub type DpdenR = crate::BitReader<Dpden>;
impl DpdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpden {
        match self.bits {
            false => Dpden::Sleepmode,
            true => Dpden::Deeppowerdown,
        }
    }
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)."]
    #[inline(always)]
    pub fn is_sleepmode(&self) -> bool {
        *self == Dpden::Sleepmode
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == Dpden::Deeppowerdown
    }
}
#[doc = "Field `DPDEN` writer - Deep power-down mode enable"]
pub type DpdenW<'a, REG> = crate::BitWriter<'a, REG, Dpden>;
impl<'a, REG> DpdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)."]
    #[inline(always)]
    pub fn sleepmode(self) -> &'a mut crate::W<REG> {
        self.variant(Dpden::Sleepmode)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Dpden::Deeppowerdown)
    }
}
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepflag {
    #[doc = "0: Read: No power-down mode entered. LPC111x is in Active mode. Write: No effect."]
    Nopowerdown = 0,
    #[doc = "1: Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    Powerdown = 1,
}
impl From<Sleepflag> for bool {
    #[inline(always)]
    fn from(variant: Sleepflag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPFLAG` reader - Sleep mode flag"]
pub type SleepflagR = crate::BitReader<Sleepflag>;
impl SleepflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepflag {
        match self.bits {
            false => Sleepflag::Nopowerdown,
            true => Sleepflag::Powerdown,
        }
    }
    #[doc = "Read: No power-down mode entered. LPC111x is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn is_nopowerdown(&self) -> bool {
        *self == Sleepflag::Nopowerdown
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Sleepflag::Powerdown
    }
}
#[doc = "Field `SLEEPFLAG` writer - Sleep mode flag"]
pub type SleepflagW<'a, REG> = crate::BitWriter<'a, REG, Sleepflag>;
impl<'a, REG> SleepflagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: No power-down mode entered. LPC111x is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn nopowerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepflag::Nopowerdown)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepflag::Powerdown)
    }
}
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpdflag {
    #[doc = "0: Read: Deep power-down mode not entered. Write: No effect."]
    Nodeeppowerdown = 0,
    #[doc = "1: Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    Deeppowerdown = 1,
}
impl From<Dpdflag> for bool {
    #[inline(always)]
    fn from(variant: Dpdflag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDFLAG` reader - Deep power-down flag"]
pub type DpdflagR = crate::BitReader<Dpdflag>;
impl DpdflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpdflag {
        match self.bits {
            false => Dpdflag::Nodeeppowerdown,
            true => Dpdflag::Deeppowerdown,
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn is_nodeeppowerdown(&self) -> bool {
        *self == Dpdflag::Nodeeppowerdown
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == Dpdflag::Deeppowerdown
    }
}
#[doc = "Field `DPDFLAG` writer - Deep power-down flag"]
pub type DpdflagW<'a, REG> = crate::BitWriter<'a, REG, Dpdflag>;
impl<'a, REG> DpdflagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn nodeeppowerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdflag::Nodeeppowerdown)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdflag::Deeppowerdown)
    }
}
impl R {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    pub fn dpden(&self) -> DpdenR {
        DpdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SleepflagR {
        SleepflagR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DpdflagR {
        DpdflagR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpden(&mut self) -> DpdenW<PconSpec> {
        DpdenW::new(self, 1)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    #[must_use]
    pub fn sleepflag(&mut self) -> SleepflagW<PconSpec> {
        SleepflagW::new(self, 8)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpdflag(&mut self) -> DpdflagW<PconSpec> {
        DpdflagW::new(self, 11)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconSpec;
impl crate::RegisterSpec for PconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PconSpec {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PconSpec {
    const RESET_VALUE: u32 = 0;
}
