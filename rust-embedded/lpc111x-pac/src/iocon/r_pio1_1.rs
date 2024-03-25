#[doc = "Register `R_PIO1_1` reader"]
pub type R = crate::R<RPio1_1Spec>;
#[doc = "Register `R_PIO1_1` writer"]
pub type W = crate::W<RPio1_1Spec>;
#[doc = "Selects pin function. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: Selects function R. This function is reserved. Select one of the alternate functions below."]
    SelectsFunctionR_ = 0,
    #[doc = "1: Selects function PIO1_1."]
    SelectsFunctionPio = 1,
    #[doc = "2: Selects function AD2."]
    SelectsFunctionAd2 = 2,
    #[doc = "3: Selects function CT32B1_MAT0."]
    SelectsFunctionCt3 = 3,
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
            0 => Some(Func::SelectsFunctionR_),
            1 => Some(Func::SelectsFunctionPio),
            2 => Some(Func::SelectsFunctionAd2),
            3 => Some(Func::SelectsFunctionCt3),
            _ => None,
        }
    }
    #[doc = "Selects function R. This function is reserved. Select one of the alternate functions below."]
    #[inline(always)]
    pub fn is_selects_function_r_(&self) -> bool {
        *self == Func::SelectsFunctionR_
    }
    #[doc = "Selects function PIO1_1."]
    #[inline(always)]
    pub fn is_selects_function_pio(&self) -> bool {
        *self == Func::SelectsFunctionPio
    }
    #[doc = "Selects function AD2."]
    #[inline(always)]
    pub fn is_selects_function_ad2(&self) -> bool {
        *self == Func::SelectsFunctionAd2
    }
    #[doc = "Selects function CT32B1_MAT0."]
    #[inline(always)]
    pub fn is_selects_function_ct3(&self) -> bool {
        *self == Func::SelectsFunctionCt3
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. All other values are reserved."]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects function R. This function is reserved. Select one of the alternate functions below."]
    #[inline(always)]
    pub fn selects_function_r_(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsFunctionR_)
    }
    #[doc = "Selects function PIO1_1."]
    #[inline(always)]
    pub fn selects_function_pio(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsFunctionPio)
    }
    #[doc = "Selects function AD2."]
    #[inline(always)]
    pub fn selects_function_ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsFunctionAd2)
    }
    #[doc = "Selects function CT32B1_MAT0."]
    #[inline(always)]
    pub fn selects_function_ct3(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SelectsFunctionCt3)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    InactiveNoPullDo = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PullDownResistorE = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PullUpResistorEna = 2,
    #[doc = "3: Repeater mode."]
    RepeaterMode_ = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::InactiveNoPullDo,
            1 => Mode::PullDownResistorE,
            2 => Mode::PullUpResistorEna,
            3 => Mode::RepeaterMode_,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == Mode::InactiveNoPullDo
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == Mode::PullDownResistorE
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == Mode::PullUpResistorEna
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == Mode::RepeaterMode_
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::InactiveNoPullDo)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullDownResistorE)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullUpResistorEna)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater_mode_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::RepeaterMode_)
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hys {
    #[doc = "0: Disable."]
    Disable_ = 0,
    #[doc = "1: Enable."]
    Enable_ = 1,
}
impl From<Hys> for bool {
    #[inline(always)]
    fn from(variant: Hys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYS` reader - Hysteresis."]
pub type HysR = crate::BitReader<Hys>;
impl HysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hys {
        match self.bits {
            false => Hys::Disable_,
            true => Hys::Enable_,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == Hys::Disable_
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable_(&self) -> bool {
        *self == Hys::Enable_
    }
}
#[doc = "Field `HYS` writer - Hysteresis."]
pub type HysW<'a, REG> = crate::BitWriter<'a, REG, Hys>;
impl<'a, REG> HysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Disable_)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable_(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Enable_)
    }
}
#[doc = "Selects Analog/Digital mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admode {
    #[doc = "0: Analog input mode"]
    AnalogInputMode = 0,
    #[doc = "1: Digital functional mode"]
    DigitalFunctionalM = 1,
}
impl From<Admode> for bool {
    #[inline(always)]
    fn from(variant: Admode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMODE` reader - Selects Analog/Digital mode"]
pub type AdmodeR = crate::BitReader<Admode>;
impl AdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admode {
        match self.bits {
            false => Admode::AnalogInputMode,
            true => Admode::DigitalFunctionalM,
        }
    }
    #[doc = "Analog input mode"]
    #[inline(always)]
    pub fn is_analog_input_mode(&self) -> bool {
        *self == Admode::AnalogInputMode
    }
    #[doc = "Digital functional mode"]
    #[inline(always)]
    pub fn is_digital_functional_m(&self) -> bool {
        *self == Admode::DigitalFunctionalM
    }
}
#[doc = "Field `ADMODE` writer - Selects Analog/Digital mode"]
pub type AdmodeW<'a, REG> = crate::BitWriter<'a, REG, Admode>;
impl<'a, REG> AdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog input mode"]
    #[inline(always)]
    pub fn analog_input_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Admode::AnalogInputMode)
    }
    #[doc = "Digital functional mode"]
    #[inline(always)]
    pub fn digital_functional_m(self) -> &'a mut crate::W<REG> {
        self.variant(Admode::DigitalFunctionalM)
    }
}
#[doc = "Selects pseudo open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Od {
    #[doc = "0: Standard GPIO output"]
    StandardGpioOutput = 0,
    #[doc = "1: Open-drain output"]
    OpenDrainOutput = 1,
}
impl From<Od> for bool {
    #[inline(always)]
    fn from(variant: Od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Selects pseudo open-drain mode."]
pub type OdR = crate::BitReader<Od>;
impl OdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Od {
        match self.bits {
            false => Od::StandardGpioOutput,
            true => Od::OpenDrainOutput,
        }
    }
    #[doc = "Standard GPIO output"]
    #[inline(always)]
    pub fn is_standard_gpio_output(&self) -> bool {
        *self == Od::StandardGpioOutput
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_open_drain_output(&self) -> bool {
        *self == Od::OpenDrainOutput
    }
}
#[doc = "Field `OD` writer - Selects pseudo open-drain mode."]
pub type OdW<'a, REG> = crate::BitWriter<'a, REG, Od>;
impl<'a, REG> OdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard GPIO output"]
    #[inline(always)]
    pub fn standard_gpio_output(self) -> &'a mut crate::W<REG> {
        self.variant(Od::StandardGpioOutput)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn open_drain_output(self) -> &'a mut crate::W<REG> {
        self.variant(Od::OpenDrainOutput)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HysR {
        HysR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode"]
    #[inline(always)]
    pub fn admode(&self) -> AdmodeR {
        AdmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<RPio1_1Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<RPio1_1Spec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HysW<RPio1_1Spec> {
        HysW::new(self, 5)
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> AdmodeW<RPio1_1Spec> {
        AdmodeW::new(self, 7)
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OdW<RPio1_1Spec> {
        OdW::new(self, 10)
    }
}
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r_pio1_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_pio1_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPio1_1Spec;
impl crate::RegisterSpec for RPio1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_pio1_1::R`](R) reader structure"]
impl crate::Readable for RPio1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`r_pio1_1::W`](W) writer structure"]
impl crate::Writable for RPio1_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R_PIO1_1 to value 0xd0"]
impl crate::Resettable for RPio1_1Spec {
    const RESET_VALUE: u32 = 0xd0;
}
