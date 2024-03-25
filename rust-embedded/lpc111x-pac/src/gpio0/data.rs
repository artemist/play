#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA0` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data0R = crate::BitReader;
#[doc = "Field `DATA0` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data1R = crate::BitReader;
#[doc = "Field `DATA1` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data2R = crate::BitReader;
#[doc = "Field `DATA2` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data3R = crate::BitReader;
#[doc = "Field `DATA3` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA4` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data4R = crate::BitReader;
#[doc = "Field `DATA4` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA5` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data5R = crate::BitReader;
#[doc = "Field `DATA5` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA6` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data6R = crate::BitReader;
#[doc = "Field `DATA6` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA7` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data7R = crate::BitReader;
#[doc = "Field `DATA7` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA8` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data8R = crate::BitReader;
#[doc = "Field `DATA8` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA9` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data9R = crate::BitReader;
#[doc = "Field `DATA9` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA10` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data10R = crate::BitReader;
#[doc = "Field `DATA10` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA11` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data11R = crate::BitReader;
#[doc = "Field `DATA11` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type Data11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data9(&self) -> Data9R {
        Data9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data10(&self) -> Data10R {
        Data10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data11(&self) -> Data11R {
        Data11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<DataSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bit 1 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<DataSpec> {
        Data1W::new(self, 1)
    }
    #[doc = "Bit 2 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> Data2W<DataSpec> {
        Data2W::new(self, 2)
    }
    #[doc = "Bit 3 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> Data3W<DataSpec> {
        Data3W::new(self, 3)
    }
    #[doc = "Bit 4 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> Data4W<DataSpec> {
        Data4W::new(self, 4)
    }
    #[doc = "Bit 5 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> Data5W<DataSpec> {
        Data5W::new(self, 5)
    }
    #[doc = "Bit 6 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> Data6W<DataSpec> {
        Data6W::new(self, 6)
    }
    #[doc = "Bit 7 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> Data7W<DataSpec> {
        Data7W::new(self, 7)
    }
    #[doc = "Bit 8 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> Data8W<DataSpec> {
        Data8W::new(self, 8)
    }
    #[doc = "Bit 9 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data9(&mut self) -> Data9W<DataSpec> {
        Data9W::new(self, 9)
    }
    #[doc = "Bit 10 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data10(&mut self) -> Data10W<DataSpec> {
        Data10W::new(self, 10)
    }
    #[doc = "Bit 11 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    #[must_use]
    pub fn data11(&mut self) -> Data11W<DataSpec> {
        Data11W::new(self, 11)
    }
}
#[doc = "Port n data register for pins PIOn_0 to PIOn_11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
