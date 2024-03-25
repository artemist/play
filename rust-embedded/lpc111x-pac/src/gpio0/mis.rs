#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `MASK0` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask0R = crate::BitReader;
#[doc = "Field `MASK1` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask1R = crate::BitReader;
#[doc = "Field `MASK2` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask2R = crate::BitReader;
#[doc = "Field `MASK3` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask3R = crate::BitReader;
#[doc = "Field `MASK4` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask4R = crate::BitReader;
#[doc = "Field `MASK5` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask5R = crate::BitReader;
#[doc = "Field `MASK6` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask6R = crate::BitReader;
#[doc = "Field `MASK7` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask7R = crate::BitReader;
#[doc = "Field `MASK8` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask8R = crate::BitReader;
#[doc = "Field `MASK9` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask9R = crate::BitReader;
#[doc = "Field `MASK10` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask10R = crate::BitReader;
#[doc = "Field `MASK11` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type Mask11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask0(&self) -> Mask0R {
        Mask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask1(&self) -> Mask1R {
        Mask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask2(&self) -> Mask2R {
        Mask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask3(&self) -> Mask3R {
        Mask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask4(&self) -> Mask4R {
        Mask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask5(&self) -> Mask5R {
        Mask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask6(&self) -> Mask6R {
        Mask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask7(&self) -> Mask7R {
        Mask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask8(&self) -> Mask8R {
        Mask8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask9(&self) -> Mask9R {
        Mask9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask10(&self) -> Mask10R {
        Mask10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask11(&self) -> Mask11R {
        Mask11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Masked interrupt status register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
