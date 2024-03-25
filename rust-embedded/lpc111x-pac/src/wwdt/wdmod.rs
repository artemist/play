#[doc = "Register `WDMOD` reader"]
pub type R = crate::R<WdmodSpec>;
#[doc = "Register `WDMOD` writer"]
pub type W = crate::W<WdmodSpec>;
#[doc = "Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wden {
    #[doc = "0: The watchdog timer is stopped."]
    Stopped = 0,
    #[doc = "1: The watchdog timer is running."]
    Run = 1,
}
impl From<Wden> for bool {
    #[inline(always)]
    fn from(variant: Wden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDEN` reader - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
pub type WdenR = crate::BitReader<Wden>;
impl WdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wden {
        match self.bits {
            false => Wden::Stopped,
            true => Wden::Run,
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Wden::Stopped
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Wden::Run
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
pub type WdenW<'a, REG> = crate::BitWriter<'a, REG, Wden>;
impl<'a, REG> WdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Wden::Stopped)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Wden::Run)
    }
}
#[doc = "Watchdog reset enable bit. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdreset {
    #[doc = "0: A watchdog timeout will not cause a chip reset."]
    Noreset = 0,
    #[doc = "1: A watchdog timeout will cause a chip reset."]
    Reset = 1,
}
impl From<Wdreset> for bool {
    #[inline(always)]
    fn from(variant: Wdreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. This bit is Set Only."]
pub type WdresetR = crate::BitReader<Wdreset>;
impl WdresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdreset {
        match self.bits {
            false => Wdreset::Noreset,
            true => Wdreset::Reset,
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn is_noreset(&self) -> bool {
        *self == Wdreset::Noreset
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Wdreset::Reset
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. This bit is Set Only."]
pub type WdresetW<'a, REG> = crate::BitWriter<'a, REG, Wdreset>;
impl<'a, REG> WdresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn noreset(self) -> &'a mut crate::W<REG> {
        self.variant(Wdreset::Noreset)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wdreset::Reset)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
pub type WdtofR = crate::BitReader;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
pub type WdtofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDINT` reader - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
pub type WdintR = crate::BitReader;
#[doc = "Field `WDINT` writer - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
pub type WdintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog update mode. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdprotect {
    #[doc = "0: The watchdog reload value (WDTC) can be changed at any time."]
    Anytime = 0,
    #[doc = "1: The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    Lowcounter = 1,
}
impl From<Wdprotect> for bool {
    #[inline(always)]
    fn from(variant: Wdprotect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDPROTECT` reader - Watchdog update mode. This bit is Set Only."]
pub type WdprotectR = crate::BitReader<Wdprotect>;
impl WdprotectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdprotect {
        match self.bits {
            false => Wdprotect::Anytime,
            true => Wdprotect::Lowcounter,
        }
    }
    #[doc = "The watchdog reload value (WDTC) can be changed at any time."]
    #[inline(always)]
    pub fn is_anytime(&self) -> bool {
        *self == Wdprotect::Anytime
    }
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    #[inline(always)]
    pub fn is_lowcounter(&self) -> bool {
        *self == Wdprotect::Lowcounter
    }
}
#[doc = "Field `WDPROTECT` writer - Watchdog update mode. This bit is Set Only."]
pub type WdprotectW<'a, REG> = crate::BitWriter<'a, REG, Wdprotect>;
impl<'a, REG> WdprotectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The watchdog reload value (WDTC) can be changed at any time."]
    #[inline(always)]
    pub fn anytime(self) -> &'a mut crate::W<REG> {
        self.variant(Wdprotect::Anytime)
    }
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    #[inline(always)]
    pub fn lowcounter(self) -> &'a mut crate::W<REG> {
        self.variant(Wdprotect::Lowcounter)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
    #[inline(always)]
    pub fn wden(&self) -> WdenR {
        WdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wdreset(&self) -> WdresetR {
        WdresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WdtofR {
        WdtofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&self) -> WdintR {
        WdintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WdprotectR {
        WdprotectR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WdenW<WdmodSpec> {
        WdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline(always)]
    #[must_use]
    pub fn wdreset(&mut self) -> WdresetW<WdmodSpec> {
        WdresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdtof(&mut self) -> WdtofW<WdmodSpec> {
        WdtofW::new(self, 2)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn wdint(&mut self) -> WdintW<WdmodSpec> {
        WdintW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline(always)]
    #[must_use]
    pub fn wdprotect(&mut self) -> WdprotectW<WdmodSpec> {
        WdprotectW::new(self, 4)
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdmodSpec;
impl crate::RegisterSpec for WdmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdmod::R`](R) reader structure"]
impl crate::Readable for WdmodSpec {}
#[doc = "`write(|w| ..)` method takes [`wdmod::W`](W) writer structure"]
impl crate::Writable for WdmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDMOD to value 0"]
impl crate::Resettable for WdmodSpec {
    const RESET_VALUE: u32 = 0;
}
