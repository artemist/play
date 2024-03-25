#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RAWST0` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst0R = crate::BitReader;
#[doc = "Field `RAWST1` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst1R = crate::BitReader;
#[doc = "Field `RAWST2` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst2R = crate::BitReader;
#[doc = "Field `RAWST3` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst3R = crate::BitReader;
#[doc = "Field `RAWST4` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst4R = crate::BitReader;
#[doc = "Field `RAWST5` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst5R = crate::BitReader;
#[doc = "Field `RAWST6` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst6R = crate::BitReader;
#[doc = "Field `RAWST7` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst7R = crate::BitReader;
#[doc = "Field `RAWST8` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst8R = crate::BitReader;
#[doc = "Field `RAWST9` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst9R = crate::BitReader;
#[doc = "Field `RAWST10` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst10R = crate::BitReader;
#[doc = "Field `RAWST11` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type Rawst11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst0(&self) -> Rawst0R {
        Rawst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst1(&self) -> Rawst1R {
        Rawst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst2(&self) -> Rawst2R {
        Rawst2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst3(&self) -> Rawst3R {
        Rawst3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst4(&self) -> Rawst4R {
        Rawst4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst5(&self) -> Rawst5R {
        Rawst5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst6(&self) -> Rawst6R {
        Rawst6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst7(&self) -> Rawst7R {
        Rawst7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst8(&self) -> Rawst8R {
        Rawst8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst9(&self) -> Rawst9R {
        Rawst9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst10(&self) -> Rawst10R {
        Rawst10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst11(&self) -> Rawst11R {
        Rawst11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
