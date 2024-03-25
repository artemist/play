#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashtim {
    #[doc = "1: 1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    _1SystemClockFlash = 1,
    #[doc = "2: 2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    _2SystemClocksFlas = 2,
    #[doc = "3: 3 system clocks flash access time (for system clock frequencies of up to 50 MHz)."]
    _3SystemClocksFlas = 3,
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(variant: Flashtim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashtim {
    type Ux = u8;
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub type FlashtimR = crate::FieldReader<Flashtim>;
impl FlashtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flashtim> {
        match self.bits {
            1 => Some(Flashtim::_1SystemClockFlash),
            2 => Some(Flashtim::_2SystemClocksFlas),
            3 => Some(Flashtim::_3SystemClocksFlas),
            _ => None,
        }
    }
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    #[inline(always)]
    pub fn is_1_system_clock_flash(&self) -> bool {
        *self == Flashtim::_1SystemClockFlash
    }
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    #[inline(always)]
    pub fn is_2_system_clocks_flas(&self) -> bool {
        *self == Flashtim::_2SystemClocksFlas
    }
    #[doc = "3 system clocks flash access time (for system clock frequencies of up to 50 MHz)."]
    #[inline(always)]
    pub fn is_3_system_clocks_flas(&self) -> bool {
        *self == Flashtim::_3SystemClocksFlas
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub type FlashtimW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flashtim>;
impl<'a, REG> FlashtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    #[inline(always)]
    pub fn _1_system_clock_flash(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_1SystemClockFlash)
    }
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    #[inline(always)]
    pub fn _2_system_clocks_flas(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_2SystemClocksFlas)
    }
    #[doc = "3 system clocks flash access time (for system clock frequencies of up to 50 MHz)."]
    #[inline(always)]
    pub fn _3_system_clocks_flas(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_3SystemClocksFlas)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&self) -> FlashtimR {
        FlashtimR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    #[must_use]
    pub fn flashtim(&mut self) -> FlashtimW<FlashcfgSpec> {
        FlashtimW::new(self, 0)
    }
}
#[doc = "Flash memory access time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcfgSpec;
impl crate::RegisterSpec for FlashcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FlashcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FlashcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCFG to value 0"]
impl crate::Resettable for FlashcfgSpec {
    const RESET_VALUE: u32 = 0;
}
