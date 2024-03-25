#[doc = "Register `SYSPLLSTAT` reader"]
pub type R = crate::R<SyspllstatSpec>;
#[doc = "PLL lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: PLL not locked"]
    PllNotLocked = 0,
    #[doc = "1: PLL locked"]
    PllLocked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - PLL lock status"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::PllNotLocked,
            true => Lock::PllLocked,
        }
    }
    #[doc = "PLL not locked"]
    #[inline(always)]
    pub fn is_pll_not_locked(&self) -> bool {
        *self == Lock::PllNotLocked
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_pll_locked(&self) -> bool {
        *self == Lock::PllLocked
    }
}
impl R {
    #[doc = "Bit 0 - PLL lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
#[doc = "System PLL status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllstatSpec;
impl crate::RegisterSpec for SyspllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllstat::R`](R) reader structure"]
impl crate::Readable for SyspllstatSpec {}
#[doc = "`reset()` method sets SYSPLLSTAT to value 0"]
impl crate::Resettable for SyspllstatSpec {
    const RESET_VALUE: u32 = 0;
}
