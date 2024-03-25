#[doc = "Register `SYSRSTSTAT` reader"]
pub type R = crate::R<SysrststatSpec>;
#[doc = "POR reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Por {
    #[doc = "0: No POR detected."]
    NoPorDetected_ = 0,
    #[doc = "1: POR detected. Writing a one clears this reset."]
    PorDetectedWritin = 1,
}
impl From<Por> for bool {
    #[inline(always)]
    fn from(variant: Por) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - POR reset status"]
pub type PorR = crate::BitReader<Por>;
impl PorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Por {
        match self.bits {
            false => Por::NoPorDetected_,
            true => Por::PorDetectedWritin,
        }
    }
    #[doc = "No POR detected."]
    #[inline(always)]
    pub fn is_no_por_detected_(&self) -> bool {
        *self == Por::NoPorDetected_
    }
    #[doc = "POR detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn is_por_detected_writin(&self) -> bool {
        *self == Por::PorDetectedWritin
    }
}
#[doc = "Status of the external RESET pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrst {
    #[doc = "0: No RESET event detected."]
    NoResetEventDetec = 0,
    #[doc = "1: RESET detected. Writing a one clears this reset."]
    ResetDetectedWrit = 1,
}
impl From<Extrst> for bool {
    #[inline(always)]
    fn from(variant: Extrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTRST` reader - Status of the external RESET pin."]
pub type ExtrstR = crate::BitReader<Extrst>;
impl ExtrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extrst {
        match self.bits {
            false => Extrst::NoResetEventDetec,
            true => Extrst::ResetDetectedWrit,
        }
    }
    #[doc = "No RESET event detected."]
    #[inline(always)]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == Extrst::NoResetEventDetec
    }
    #[doc = "RESET detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn is_reset_detected_writ(&self) -> bool {
        *self == Extrst::ResetDetectedWrit
    }
}
#[doc = "Status of the Watchdog reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: No WDT reset detected."]
    NoWdtResetDetecte = 0,
    #[doc = "1: WDT reset detected. Writing a one clears this reset."]
    WdtResetDetected_ = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Status of the Watchdog reset"]
pub type WdtR = crate::BitReader<Wdt>;
impl WdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt {
        match self.bits {
            false => Wdt::NoWdtResetDetecte,
            true => Wdt::WdtResetDetected_,
        }
    }
    #[doc = "No WDT reset detected."]
    #[inline(always)]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == Wdt::NoWdtResetDetecte
    }
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn is_wdt_reset_detected_(&self) -> bool {
        *self == Wdt::WdtResetDetected_
    }
}
#[doc = "Status of the Brown-out detect reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bod {
    #[doc = "0: No BOD reset detected."]
    NoBodResetDetecte = 0,
    #[doc = "1: BOD reset detected. Writing a one clears this reset."]
    BodResetDetected_ = 1,
}
impl From<Bod> for bool {
    #[inline(always)]
    fn from(variant: Bod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD` reader - Status of the Brown-out detect reset"]
pub type BodR = crate::BitReader<Bod>;
impl BodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bod {
        match self.bits {
            false => Bod::NoBodResetDetecte,
            true => Bod::BodResetDetected_,
        }
    }
    #[doc = "No BOD reset detected."]
    #[inline(always)]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == Bod::NoBodResetDetecte
    }
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn is_bod_reset_detected_(&self) -> bool {
        *self == Bod::BodResetDetected_
    }
}
#[doc = "Status of the software system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrst {
    #[doc = "0: No System reset detected."]
    NoSystemResetDete = 0,
    #[doc = "1: System reset detected. Writing a one clears this reset."]
    SystemResetDetecte = 1,
}
impl From<Sysrst> for bool {
    #[inline(always)]
    fn from(variant: Sysrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRST` reader - Status of the software system reset"]
pub type SysrstR = crate::BitReader<Sysrst>;
impl SysrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrst {
        match self.bits {
            false => Sysrst::NoSystemResetDete,
            true => Sysrst::SystemResetDetecte,
        }
    }
    #[doc = "No System reset detected."]
    #[inline(always)]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == Sysrst::NoSystemResetDete
    }
    #[doc = "System reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == Sysrst::SystemResetDetecte
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin."]
    #[inline(always)]
    pub fn extrst(&self) -> ExtrstR {
        ExtrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&self) -> BodR {
        BodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&self) -> SysrstR {
        SysrstR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "System reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrststat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrststatSpec;
impl crate::RegisterSpec for SysrststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysrststat::R`](R) reader structure"]
impl crate::Readable for SysrststatSpec {}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0"]
impl crate::Resettable for SysrststatSpec {
    const RESET_VALUE: u32 = 0;
}
