#[doc = "Register `STARTRSRP0CLR` writer"]
pub type W = crate::W<Startrsrp0clrSpec>;
#[doc = "Field `RSRPIO0_0` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_1` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_2` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_3` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_4` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_5` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_6` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_7` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_8` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_9` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_10` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO0_11` writer - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio0_11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSRPIO1_0` writer - Start signal reset for start logic input PIO1_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
pub type Rsrpio1_0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_0(&mut self) -> Rsrpio0_0W<Startrsrp0clrSpec> {
        Rsrpio0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_1(&mut self) -> Rsrpio0_1W<Startrsrp0clrSpec> {
        Rsrpio0_1W::new(self, 1)
    }
    #[doc = "Bit 2 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_2(&mut self) -> Rsrpio0_2W<Startrsrp0clrSpec> {
        Rsrpio0_2W::new(self, 2)
    }
    #[doc = "Bit 3 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_3(&mut self) -> Rsrpio0_3W<Startrsrp0clrSpec> {
        Rsrpio0_3W::new(self, 3)
    }
    #[doc = "Bit 4 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_4(&mut self) -> Rsrpio0_4W<Startrsrp0clrSpec> {
        Rsrpio0_4W::new(self, 4)
    }
    #[doc = "Bit 5 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_5(&mut self) -> Rsrpio0_5W<Startrsrp0clrSpec> {
        Rsrpio0_5W::new(self, 5)
    }
    #[doc = "Bit 6 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_6(&mut self) -> Rsrpio0_6W<Startrsrp0clrSpec> {
        Rsrpio0_6W::new(self, 6)
    }
    #[doc = "Bit 7 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_7(&mut self) -> Rsrpio0_7W<Startrsrp0clrSpec> {
        Rsrpio0_7W::new(self, 7)
    }
    #[doc = "Bit 8 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_8(&mut self) -> Rsrpio0_8W<Startrsrp0clrSpec> {
        Rsrpio0_8W::new(self, 8)
    }
    #[doc = "Bit 9 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_9(&mut self) -> Rsrpio0_9W<Startrsrp0clrSpec> {
        Rsrpio0_9W::new(self, 9)
    }
    #[doc = "Bit 10 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_10(&mut self) -> Rsrpio0_10W<Startrsrp0clrSpec> {
        Rsrpio0_10W::new(self, 10)
    }
    #[doc = "Bit 11 - Start signal reset for start logic input PIO0_n:PIO0_11 to PIO0_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio0_11(&mut self) -> Rsrpio0_11W<Startrsrp0clrSpec> {
        Rsrpio0_11W::new(self, 11)
    }
    #[doc = "Bit 12 - Start signal reset for start logic input PIO1_0 0 = Do nothing. 1 = Writing 1 resets the start signal."]
    #[inline(always)]
    #[must_use]
    pub fn rsrpio1_0(&mut self) -> Rsrpio1_0W<Startrsrp0clrSpec> {
        Rsrpio1_0W::new(self, 12)
    }
}
#[doc = "Start logic reset register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startrsrp0clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Startrsrp0clrSpec;
impl crate::RegisterSpec for Startrsrp0clrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`startrsrp0clr::W`](W) writer structure"]
impl crate::Writable for Startrsrp0clrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTRSRP0CLR to value 0"]
impl crate::Resettable for Startrsrp0clrSpec {
    const RESET_VALUE: u32 = 0;
}
