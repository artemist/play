#[doc = "Register `PIO0_4` reader"]
pub type R = crate::R<Pio0_4Spec>;
#[doc = "Register `PIO0_4` writer"]
pub type W = crate::W<Pio0_4Spec>;
#[doc = "Selects pin function. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: Selects function PIO0_4 (open-drain pin)."]
    SelectsFunctionPio = 0,
    #[doc = "1: Selects I2C function SCL (open-drain pin)."]
    SelectsI2cFunction = 1,
}
impl From<Func> for u8 {
    #[inline(always)]
    fn from(variant: Func) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Func {
    type Ux = u8;
}
#[doc = "Field `FUNC` reader - Selects pin function. All other values are reserved."]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::SelectsFunctionPio),
            1 => Some(Func::SelectsI2cFunction),
            _ => None,
        }
    }
    #[doc = "Selects function PIO0_4 (open-drain pin)."]
    #[inline(always)]
    pub fn is_selects_function_pio(&self) -> bool {
        *self == Func::SelectsFunctionPio
    }
    #[doc = "Selects I2C function SCL (open-drain pin)."]
    #[inline(always)]
    pub fn is_selects_i2c_function(&self) -> bool {
        *self == Func::SelectsI2cFunction
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. All other values are reserved."]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects function PIO0_4 (open-drain pin)."]
    #[inline(always)]
    pub fn selects_function_pio(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsFunctionPio)
    }
    #[doc = "Selects I2C function SCL (open-drain pin)."]
    #[inline(always)]
    pub fn selects_i2c_function(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsI2cFunction)
    }
}
#[doc = "Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cmode {
    #[doc = "0: Standard mode/Fast-mode I2C"]
    StandardI2c = 0,
    #[doc = "1: Standard I/O"]
    StandardIo = 1,
    #[doc = "2: Fast-mode Plus I2C"]
    FastPlusI2c = 2,
}
impl From<I2cmode> for u8 {
    #[inline(always)]
    fn from(variant: I2cmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2cmode {
    type Ux = u8;
}
#[doc = "Field `I2CMODE` reader - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2cmodeR = crate::FieldReader<I2cmode>;
impl I2cmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2cmode> {
        match self.bits {
            0 => Some(I2cmode::StandardI2c),
            1 => Some(I2cmode::StandardIo),
            2 => Some(I2cmode::FastPlusI2c),
            _ => None,
        }
    }
    #[doc = "Standard mode/Fast-mode I2C"]
    #[inline(always)]
    pub fn is_standard_i2c(&self) -> bool {
        *self == I2cmode::StandardI2c
    }
    #[doc = "Standard I/O"]
    #[inline(always)]
    pub fn is_standard_io(&self) -> bool {
        *self == I2cmode::StandardIo
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn is_fast_plus_i2c(&self) -> bool {
        *self == I2cmode::FastPlusI2c
    }
}
#[doc = "Field `I2CMODE` writer - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2cmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2cmode>;
impl<'a, REG> I2cmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard mode/Fast-mode I2C"]
    #[inline(always)]
    pub fn standard_i2c(self) -> &'a mut crate::W<REG> {
        self.variant(I2cmode::StandardI2c)
    }
    #[doc = "Standard I/O"]
    #[inline(always)]
    pub fn standard_io(self) -> &'a mut crate::W<REG> {
        self.variant(I2cmode::StandardIo)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_plus_i2c(self) -> &'a mut crate::W<REG> {
        self.variant(I2cmode::FastPlusI2c)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2cmodeR {
        I2cmodeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<Pio0_4Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    #[must_use]
    pub fn i2cmode(&mut self) -> I2cmodeW<Pio0_4Spec> {
        I2cmodeW::new(self, 8)
    }
}
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pio0_4Spec;
impl crate::RegisterSpec for Pio0_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio0_4::R`](R) reader structure"]
impl crate::Readable for Pio0_4Spec {}
#[doc = "`write(|w| ..)` method takes [`pio0_4::W`](W) writer structure"]
impl crate::Writable for Pio0_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIO0_4 to value 0"]
impl crate::Resettable for Pio0_4Spec {
    const RESET_VALUE: u32 = 0;
}
