#[doc = "Register `PIOPORCAP1` reader"]
pub type R = crate::R<Pioporcap1Spec>;
#[doc = "Field `CAPPIO2_8` reader - Raw reset status input PIO2_8"]
pub type Cappio2_8R = crate::BitReader;
#[doc = "Field `CAPPIO2_9` reader - Raw reset status input PIO2_9"]
pub type Cappio2_9R = crate::BitReader;
#[doc = "Field `CAPPIO2_10` reader - Raw reset status input PIO2_10"]
pub type Cappio2_10R = crate::BitReader;
#[doc = "Field `CAPPIO2_11` reader - Raw reset status input PIO2_11"]
pub type Cappio2_11R = crate::BitReader;
#[doc = "Field `CAPPIO3_0` reader - Raw reset status input PIO3_0"]
pub type Cappio3_0R = crate::BitReader;
#[doc = "Field `CAPPIO3_1` reader - Raw reset status input PIO3_1"]
pub type Cappio3_1R = crate::BitReader;
#[doc = "Field `CAPPIO3_2` reader - Raw reset status input PIO3_2"]
pub type Cappio3_2R = crate::BitReader;
#[doc = "Field `CAPPIO3_3` reader - Raw reset status input PIO3_3"]
pub type Cappio3_3R = crate::BitReader;
#[doc = "Field `CAPPIO3_4` reader - Raw reset status input PIO3_4"]
pub type Cappio3_4R = crate::BitReader;
#[doc = "Field `CAPPIO3_5` reader - Raw reset status input PIO3_5"]
pub type Cappio3_5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw reset status input PIO2_8"]
    #[inline(always)]
    pub fn cappio2_8(&self) -> Cappio2_8R {
        Cappio2_8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw reset status input PIO2_9"]
    #[inline(always)]
    pub fn cappio2_9(&self) -> Cappio2_9R {
        Cappio2_9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw reset status input PIO2_10"]
    #[inline(always)]
    pub fn cappio2_10(&self) -> Cappio2_10R {
        Cappio2_10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw reset status input PIO2_11"]
    #[inline(always)]
    pub fn cappio2_11(&self) -> Cappio2_11R {
        Cappio2_11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw reset status input PIO3_0"]
    #[inline(always)]
    pub fn cappio3_0(&self) -> Cappio3_0R {
        Cappio3_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw reset status input PIO3_1"]
    #[inline(always)]
    pub fn cappio3_1(&self) -> Cappio3_1R {
        Cappio3_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw reset status input PIO3_2"]
    #[inline(always)]
    pub fn cappio3_2(&self) -> Cappio3_2R {
        Cappio3_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw reset status input PIO3_3"]
    #[inline(always)]
    pub fn cappio3_3(&self) -> Cappio3_3R {
        Cappio3_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw reset status input PIO3_4"]
    #[inline(always)]
    pub fn cappio3_4(&self) -> Cappio3_4R {
        Cappio3_4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw reset status input PIO3_5"]
    #[inline(always)]
    pub fn cappio3_5(&self) -> Cappio3_5R {
        Cappio3_5R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "POR captured PIO status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioporcap1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pioporcap1Spec;
impl crate::RegisterSpec for Pioporcap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pioporcap1::R`](R) reader structure"]
impl crate::Readable for Pioporcap1Spec {}
#[doc = "`reset()` method sets PIOPORCAP1 to value 0"]
impl crate::Resettable for Pioporcap1Spec {
    const RESET_VALUE: u32 = 0;
}
