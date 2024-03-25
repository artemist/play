#[doc = "Register `SYSMEMREMAP` reader"]
pub type R = crate::R<SysmemremapSpec>;
#[doc = "Register `SYSMEMREMAP` writer"]
pub type W = crate::W<SysmemremapSpec>;
#[doc = "System memory remap\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Map {
    #[doc = "0: Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BootLoaderModeIn = 0,
    #[doc = "1: User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    UserRamModeInter = 1,
    #[doc = "2: User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    UserFlashModeInt = 2,
}
impl From<Map> for u8 {
    #[inline(always)]
    fn from(variant: Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Map {
    type Ux = u8;
}
#[doc = "Field `MAP` reader - System memory remap"]
pub type MapR = crate::FieldReader<Map>;
impl MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Map> {
        match self.bits {
            0 => Some(Map::BootLoaderModeIn),
            1 => Some(Map::UserRamModeInter),
            2 => Some(Map::UserFlashModeInt),
            _ => None,
        }
    }
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline(always)]
    pub fn is_boot_loader_mode_in(&self) -> bool {
        *self == Map::BootLoaderModeIn
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline(always)]
    pub fn is_user_ram_mode_inter(&self) -> bool {
        *self == Map::UserRamModeInter
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline(always)]
    pub fn is_user_flash_mode_int(&self) -> bool {
        *self == Map::UserFlashModeInt
    }
}
#[doc = "Field `MAP` writer - System memory remap"]
pub type MapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Map>;
impl<'a, REG> MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline(always)]
    pub fn boot_loader_mode_in(self) -> &'a mut crate::W<REG> {
        self.variant(Map::BootLoaderModeIn)
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline(always)]
    pub fn user_ram_mode_inter(self) -> &'a mut crate::W<REG> {
        self.variant(Map::UserRamModeInter)
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline(always)]
    pub fn user_flash_mode_int(self) -> &'a mut crate::W<REG> {
        self.variant(Map::UserFlashModeInt)
    }
}
impl R {
    #[doc = "Bits 0:1 - System memory remap"]
    #[inline(always)]
    pub fn map(&self) -> MapR {
        MapR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System memory remap"]
    #[inline(always)]
    #[must_use]
    pub fn map(&mut self) -> MapW<SysmemremapSpec> {
        MapW::new(self, 0)
    }
}
#[doc = "System memory remap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysmemremap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysmemremap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemremapSpec;
impl crate::RegisterSpec for SysmemremapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmemremap::R`](R) reader structure"]
impl crate::Readable for SysmemremapSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmemremap::W`](W) writer structure"]
impl crate::Writable for SysmemremapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSMEMREMAP to value 0x02"]
impl crate::Resettable for SysmemremapSpec {
    const RESET_VALUE: u32 = 0x02;
}
