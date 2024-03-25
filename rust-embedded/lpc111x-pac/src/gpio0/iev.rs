#[doc = "Register `IEV` reader"]
pub type R = crate::R<IevSpec>;
#[doc = "Register `IEV` writer"]
pub type W = crate::W<IevSpec>;
#[doc = "Field `IEV0` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev0R = crate::BitReader;
#[doc = "Field `IEV0` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV1` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev1R = crate::BitReader;
#[doc = "Field `IEV1` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV2` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev2R = crate::BitReader;
#[doc = "Field `IEV2` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV3` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev3R = crate::BitReader;
#[doc = "Field `IEV3` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV4` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev4R = crate::BitReader;
#[doc = "Field `IEV4` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV5` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev5R = crate::BitReader;
#[doc = "Field `IEV5` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV6` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev6R = crate::BitReader;
#[doc = "Field `IEV6` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV7` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev7R = crate::BitReader;
#[doc = "Field `IEV7` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV8` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev8R = crate::BitReader;
#[doc = "Field `IEV8` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV9` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev9R = crate::BitReader;
#[doc = "Field `IEV9` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV10` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev10R = crate::BitReader;
#[doc = "Field `IEV10` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEV11` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev11R = crate::BitReader;
#[doc = "Field `IEV11` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type Iev11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev0(&self) -> Iev0R {
        Iev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev1(&self) -> Iev1R {
        Iev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev2(&self) -> Iev2R {
        Iev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev3(&self) -> Iev3R {
        Iev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev4(&self) -> Iev4R {
        Iev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev5(&self) -> Iev5R {
        Iev5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev6(&self) -> Iev6R {
        Iev6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev7(&self) -> Iev7R {
        Iev7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev8(&self) -> Iev8R {
        Iev8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev9(&self) -> Iev9R {
        Iev9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev10(&self) -> Iev10R {
        Iev10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev11(&self) -> Iev11R {
        Iev11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev0(&mut self) -> Iev0W<IevSpec> {
        Iev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev1(&mut self) -> Iev1W<IevSpec> {
        Iev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev2(&mut self) -> Iev2W<IevSpec> {
        Iev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev3(&mut self) -> Iev3W<IevSpec> {
        Iev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev4(&mut self) -> Iev4W<IevSpec> {
        Iev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev5(&mut self) -> Iev5W<IevSpec> {
        Iev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev6(&mut self) -> Iev6W<IevSpec> {
        Iev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev7(&mut self) -> Iev7W<IevSpec> {
        Iev7W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev8(&mut self) -> Iev8W<IevSpec> {
        Iev8W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev9(&mut self) -> Iev9W<IevSpec> {
        Iev9W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev10(&mut self) -> Iev10W<IevSpec> {
        Iev10W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOnIS (see Table 109), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOnIS (see Table 109), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iev11(&mut self) -> Iev11W<IevSpec> {
        Iev11W::new(self, 11)
    }
}
#[doc = "Interrupt event register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IevSpec;
impl crate::RegisterSpec for IevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iev::R`](R) reader structure"]
impl crate::Readable for IevSpec {}
#[doc = "`write(|w| ..)` method takes [`iev::W`](W) writer structure"]
impl crate::Writable for IevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IevSpec {
    const RESET_VALUE: u32 = 0;
}
