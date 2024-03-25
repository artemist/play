#[doc = "Register `STARTSRP0` reader"]
pub type R = crate::R<Startsrp0Spec>;
#[doc = "Field `SRPIO0_0` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_0R = crate::BitReader;
#[doc = "Field `SRPIO0_1` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_1R = crate::BitReader;
#[doc = "Field `SRPIO0_2` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_2R = crate::BitReader;
#[doc = "Field `SRPIO0_3` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_3R = crate::BitReader;
#[doc = "Field `SRPIO0_4` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_4R = crate::BitReader;
#[doc = "Field `SRPIO0_5` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_5R = crate::BitReader;
#[doc = "Field `SRPIO0_6` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_6R = crate::BitReader;
#[doc = "Field `SRPIO0_7` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_7R = crate::BitReader;
#[doc = "Field `SRPIO0_8` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_8R = crate::BitReader;
#[doc = "Field `SRPIO0_9` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_9R = crate::BitReader;
#[doc = "Field `SRPIO0_10` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_10R = crate::BitReader;
#[doc = "Field `SRPIO0_11` reader - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio0_11R = crate::BitReader;
#[doc = "Field `SRPIO1_0` reader - Start signal status for start logic input PIO1_0 0 = No start signal received. 1 = Start signal pending."]
pub type Srpio1_0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_0(&self) -> Srpio0_0R {
        Srpio0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_1(&self) -> Srpio0_1R {
        Srpio0_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_2(&self) -> Srpio0_2R {
        Srpio0_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_3(&self) -> Srpio0_3R {
        Srpio0_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_4(&self) -> Srpio0_4R {
        Srpio0_4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_5(&self) -> Srpio0_5R {
        Srpio0_5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_6(&self) -> Srpio0_6R {
        Srpio0_6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_7(&self) -> Srpio0_7R {
        Srpio0_7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_8(&self) -> Srpio0_8R {
        Srpio0_8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_9(&self) -> Srpio0_9R {
        Srpio0_9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_10(&self) -> Srpio0_10R {
        Srpio0_10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_11(&self) -> Srpio0_11R {
        Srpio0_11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Start signal status for start logic input PIO1_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_0(&self) -> Srpio1_0R {
        Srpio1_0R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Start logic status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startsrp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Startsrp0Spec;
impl crate::RegisterSpec for Startsrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startsrp0::R`](R) reader structure"]
impl crate::Readable for Startsrp0Spec {}
#[doc = "`reset()` method sets STARTSRP0 to value 0"]
impl crate::Resettable for Startsrp0Spec {
    const RESET_VALUE: u32 = 0;
}
