#[doc = "Register `RXD_LOC` reader"]
pub type R = crate::R<RxdLocSpec>;
#[doc = "Register `RXD_LOC` writer"]
pub type W = crate::W<RxdLocSpec>;
#[doc = "Selects pin location for the RXD function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxdloc {
    #[doc = "0: `0`"]
    Pio1_6 = 0,
    #[doc = "1: `1`"]
    Pio2_7 = 1,
    #[doc = "2: `10`"]
    Pio3_1 = 2,
    #[doc = "3: `11`"]
    Pio3_4 = 3,
}
impl From<Rxdloc> for u8 {
    #[inline(always)]
    fn from(variant: Rxdloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxdloc {
    type Ux = u8;
}
#[doc = "Field `RXDLOC` reader - Selects pin location for the RXD function."]
pub type RxdlocR = crate::FieldReader<Rxdloc>;
impl RxdlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdloc {
        match self.bits {
            0 => Rxdloc::Pio1_6,
            1 => Rxdloc::Pio2_7,
            2 => Rxdloc::Pio3_1,
            3 => Rxdloc::Pio3_4,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pio1_6(&self) -> bool {
        *self == Rxdloc::Pio1_6
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pio2_7(&self) -> bool {
        *self == Rxdloc::Pio2_7
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pio3_1(&self) -> bool {
        *self == Rxdloc::Pio3_1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pio3_4(&self) -> bool {
        *self == Rxdloc::Pio3_4
    }
}
#[doc = "Field `RXDLOC` writer - Selects pin location for the RXD function."]
pub type RxdlocW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxdloc>;
impl<'a, REG> RxdlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pio1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdloc::Pio1_6)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pio2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdloc::Pio2_7)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pio3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdloc::Pio3_1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pio3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdloc::Pio3_4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the RXD function."]
    #[inline(always)]
    pub fn rxdloc(&self) -> RxdlocR {
        RxdlocR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the RXD function."]
    #[inline(always)]
    #[must_use]
    pub fn rxdloc(&mut self) -> RxdlocW<RxdLocSpec> {
        RxdlocW::new(self, 0)
    }
}
#[doc = "RXD pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdLocSpec;
impl crate::RegisterSpec for RxdLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd_loc::R`](R) reader structure"]
impl crate::Readable for RxdLocSpec {}
#[doc = "`write(|w| ..)` method takes [`rxd_loc::W`](W) writer structure"]
impl crate::Writable for RxdLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXD_LOC to value 0"]
impl crate::Resettable for RxdLocSpec {
    const RESET_VALUE: u32 = 0;
}
