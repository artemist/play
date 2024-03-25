#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SEL` reader - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKDIV` reader - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Burst {
    #[doc = "0: Software-controlled mode: Conversions are software-controlled and require 11 clocks."]
    Swmode = 0,
    #[doc = "1: Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    Hwmode = 1,
}
impl From<Burst> for bool {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Burst mode"]
pub type BurstR = crate::BitReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            false => Burst::Swmode,
            true => Burst::Hwmode,
        }
    }
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks."]
    #[inline(always)]
    pub fn is_swmode(&self) -> bool {
        *self == Burst::Swmode
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn is_hwmode(&self) -> bool {
        *self == Burst::Hwmode
    }
}
#[doc = "Field `BURST` writer - Burst mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks."]
    #[inline(always)]
    pub fn swmode(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Swmode)
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn hwmode(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Hwmode)
    }
}
#[doc = "This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: 11 clocks / 10 bits"]
    _10bit = 0,
    #[doc = "1: 10 clocks / 9 bits"]
    _9bit = 1,
    #[doc = "2: 9 clocks / 8 bits"]
    _8bit = 2,
    #[doc = "3: 8 clocks / 7 bits"]
    _7bit = 3,
    #[doc = "4: 7 clocks / 6 bits"]
    _6bit = 4,
    #[doc = "5: 6 clocks / 5 bits"]
    _5bit = 5,
    #[doc = "6: 5 clocks / 4 bits"]
    _4bit = 6,
    #[doc = "7: 4 clocks / 3 bits"]
    _3bit = 7,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
#[doc = "Field `CLKS` reader - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::_10bit,
            1 => Clks::_9bit,
            2 => Clks::_8bit,
            3 => Clks::_7bit,
            4 => Clks::_6bit,
            5 => Clks::_5bit,
            6 => Clks::_4bit,
            7 => Clks::_3bit,
            _ => unreachable!(),
        }
    }
    #[doc = "11 clocks / 10 bits"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Clks::_10bit
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Clks::_9bit
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Clks::_8bit
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Clks::_7bit
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Clks::_6bit
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Clks::_5bit
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Clks::_4bit
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline(always)]
    pub fn is_3bit(&self) -> bool {
        *self == Clks::_3bit
    }
}
#[doc = "Field `CLKS` writer - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
pub type ClksW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Clks>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "11 clocks / 10 bits"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_10bit)
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_9bit)
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_8bit)
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_7bit)
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_6bit)
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_5bit)
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_4bit)
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline(always)]
    pub fn _3bit(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_3bit)
    }
}
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Start {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)."]
    Stop = 0,
    #[doc = "1: Start conversion now."]
    Start = 1,
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    Edgepio0_2 = 2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    Edgepio1_5 = 3,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    Edgect32b0Mat0_1 = 4,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    Edgect32b0Mat1_1 = 5,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    Edgect16b0Mat0_1 = 6,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    Edgect16b0Mat1_1 = 7,
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Start {
    type Ux = u8;
}
#[doc = "Field `START` reader - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartR = crate::FieldReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            0 => Start::Stop,
            1 => Start::Start,
            2 => Start::Edgepio0_2,
            3 => Start::Edgepio1_5,
            4 => Start::Edgect32b0Mat0_1,
            5 => Start::Edgect32b0Mat1_1,
            6 => Start::Edgect16b0Mat0_1,
            7 => Start::Edgect16b0Mat1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Start::Stop
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    #[inline(always)]
    pub fn is_edgepio0_2(&self) -> bool {
        *self == Start::Edgepio0_2
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    #[inline(always)]
    pub fn is_edgepio1_5(&self) -> bool {
        *self == Start::Edgepio1_5
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn is_edgect32b0_mat0_1(&self) -> bool {
        *self == Start::Edgect32b0Mat0_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn is_edgect32b0_mat1_1(&self) -> bool {
        *self == Start::Edgect32b0Mat1_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn is_edgect16b0_mat0_1(&self) -> bool {
        *self == Start::Edgect16b0Mat0_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn is_edgect16b0_mat1_1(&self) -> bool {
        *self == Start::Edgect16b0Mat1_1
    }
}
#[doc = "Field `START` writer - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Stop)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    #[inline(always)]
    pub fn edgepio0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgepio0_2)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    #[inline(always)]
    pub fn edgepio1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgepio1_5)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn edgect32b0_mat0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgect32b0Mat0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn edgect32b0_mat1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgect32b0Mat1_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn edgect16b0_mat0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgect16b0Mat0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn edgect16b0_mat1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Edgect16b0Mat1_1)
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    Rising = 0,
    #[doc = "1: Start conversion on a rising edge on the selected CAP/MAT signal."]
    Falling = 1,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            false => Edge::Rising,
            true => Edge::Falling,
        }
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edge::Rising
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Edge::Falling
    }
}
#[doc = "Field `EDGE` writer - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Rising)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Falling)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<CrSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<CrSpec> {
        ClkdivW::new(self, 8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BurstW<CrSpec> {
        BurstW::new(self, 16)
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> ClksW<CrSpec> {
        ClksW::new(self, 17)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 24)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<CrSpec> {
        EdgeW::new(self, 27)
    }
}
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
