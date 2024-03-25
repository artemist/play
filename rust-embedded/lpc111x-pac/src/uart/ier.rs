#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbrie {
    #[doc = "0: Disable the RDA interrupt."]
    DisableTheRdaInte = 0,
    #[doc = "1: Enable the RDA interrupt."]
    EnableTheRdaInter = 1,
}
impl From<Rbrie> for bool {
    #[inline(always)]
    fn from(variant: Rbrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
pub type RbrieR = crate::BitReader<Rbrie>;
impl RbrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbrie {
        match self.bits {
            false => Rbrie::DisableTheRdaInte,
            true => Rbrie::EnableTheRdaInter,
        }
    }
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == Rbrie::DisableTheRdaInte
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == Rbrie::EnableTheRdaInter
    }
}
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
pub type RbrieW<'a, REG> = crate::BitWriter<'a, REG, Rbrie>;
impl<'a, REG> RbrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrie::DisableTheRdaInte)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrie::EnableTheRdaInter)
    }
}
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Threie {
    #[doc = "0: Disable the THRE interrupt."]
    DisableTheThreInt = 0,
    #[doc = "1: Enable the THRE interrupt."]
    EnableTheThreInte = 1,
}
impl From<Threie> for bool {
    #[inline(always)]
    fn from(variant: Threie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREIE` reader - THRE Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
pub type ThreieR = crate::BitReader<Threie>;
impl ThreieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Threie {
        match self.bits {
            false => Threie::DisableTheThreInt,
            true => Threie::EnableTheThreInte,
        }
    }
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == Threie::DisableTheThreInt
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == Threie::EnableTheThreInte
    }
}
#[doc = "Field `THREIE` writer - THRE Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
pub type ThreieW<'a, REG> = crate::BitWriter<'a, REG, Threie>;
impl<'a, REG> ThreieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut crate::W<REG> {
        self.variant(Threie::DisableTheThreInt)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Threie::EnableTheThreInte)
    }
}
#[doc = "RX Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxlie {
    #[doc = "0: Disable the RX line status interrupts."]
    DisableTheRxLine_ = 0,
    #[doc = "1: Enable the RX line status interrupts."]
    EnableTheRxLineS = 1,
}
impl From<Rxlie> for bool {
    #[inline(always)]
    fn from(variant: Rxlie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLIE` reader - RX Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RxlieR = crate::BitReader<Rxlie>;
impl RxlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxlie {
        match self.bits {
            false => Rxlie::DisableTheRxLine_,
            true => Rxlie::EnableTheRxLineS,
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_disable_the_rx_line_(&self) -> bool {
        *self == Rxlie::DisableTheRxLine_
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_enable_the_rx_line_s(&self) -> bool {
        *self == Rxlie::EnableTheRxLineS
    }
}
#[doc = "Field `RXLIE` writer - RX Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RxlieW<'a, REG> = crate::BitWriter<'a, REG, Rxlie>;
impl<'a, REG> RxlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable_the_rx_line_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlie::DisableTheRxLine_)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable_the_rx_line_s(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlie::EnableTheRxLineS)
    }
}
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abeointen {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DisableEndOfAuto_ = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    EnableEndOfAutoB = 1,
}
impl From<Abeointen> for bool {
    #[inline(always)]
    fn from(variant: Abeointen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOINTEN` reader - Enables the end of auto-baud interrupt."]
pub type AbeointenR = crate::BitReader<Abeointen>;
impl AbeointenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abeointen {
        match self.bits {
            false => Abeointen::DisableEndOfAuto_,
            true => Abeointen::EnableEndOfAutoB,
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_disable_end_of_auto_(&self) -> bool {
        *self == Abeointen::DisableEndOfAuto_
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == Abeointen::EnableEndOfAutoB
    }
}
#[doc = "Field `ABEOINTEN` writer - Enables the end of auto-baud interrupt."]
pub type AbeointenW<'a, REG> = crate::BitWriter<'a, REG, Abeointen>;
impl<'a, REG> AbeointenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable_end_of_auto_(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointen::DisableEndOfAuto_)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable_end_of_auto_b(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointen::EnableEndOfAutoB)
    }
}
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abtointen {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DisableAutoBaudTi = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    EnableAutoBaudTim = 1,
}
impl From<Abtointen> for bool {
    #[inline(always)]
    fn from(variant: Abtointen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOINTEN` reader - Enables the auto-baud time-out interrupt."]
pub type AbtointenR = crate::BitReader<Abtointen>;
impl AbtointenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abtointen {
        match self.bits {
            false => Abtointen::DisableAutoBaudTi,
            true => Abtointen::EnableAutoBaudTim,
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_disable_auto_baud_ti(&self) -> bool {
        *self == Abtointen::DisableAutoBaudTi
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_enable_auto_baud_tim(&self) -> bool {
        *self == Abtointen::EnableAutoBaudTim
    }
}
#[doc = "Field `ABTOINTEN` writer - Enables the auto-baud time-out interrupt."]
pub type AbtointenW<'a, REG> = crate::BitWriter<'a, REG, Abtointen>;
impl<'a, REG> AbtointenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable_auto_baud_ti(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointen::DisableAutoBaudTi)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable_auto_baud_tim(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointen::EnableAutoBaudTim)
    }
}
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RbrieR {
        RbrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> ThreieR {
        ThreieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxlie(&self) -> RxlieR {
        RxlieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> AbeointenR {
        AbeointenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> AbtointenR {
        AbtointenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rbrie(&mut self) -> RbrieW<IerSpec> {
        RbrieW::new(self, 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn threie(&mut self) -> ThreieW<IerSpec> {
        ThreieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn rxlie(&mut self) -> RxlieW<IerSpec> {
        RxlieW::new(self, 2)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abeointen(&mut self) -> AbeointenW<IerSpec> {
        AbeointenW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abtointen(&mut self) -> AbtointenW<IerSpec> {
        AbtointenW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts. (DLAB=0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
