#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `DTRC` reader - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DtrcR = crate::BitReader;
#[doc = "Field `DTRC` writer - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DtrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSC` reader - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RtscR = crate::BitReader;
#[doc = "Field `RTSC` writer - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RtscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMS` reader - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
pub type LmsR = crate::BitReader;
#[doc = "Field `LMS` writer - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
pub type LmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RTS flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "0: Disable auto-rts flow control."]
    DisableAutoRtsFlo = 0,
    #[doc = "1: Enable auto-rts flow control."]
    EnableAutoRtsFlow = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - RTS flow control"]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::DisableAutoRtsFlo,
            true => Rtsen::EnableAutoRtsFlow,
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == Rtsen::DisableAutoRtsFlo
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == Rtsen::EnableAutoRtsFlow
    }
}
#[doc = "Field `RTSEN` writer - RTS flow control"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::DisableAutoRtsFlo)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::EnableAutoRtsFlow)
    }
}
#[doc = "CTS flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: Disable auto-cts flow control."]
    DisableAutoCtsFlo = 0,
    #[doc = "1: Enable auto-cts flow control."]
    EnableAutoCtsFlow = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS flow control"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::DisableAutoCtsFlo,
            true => Ctsen::EnableAutoCtsFlow,
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == Ctsen::DisableAutoCtsFlo
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == Ctsen::EnableAutoCtsFlow
    }
}
#[doc = "Field `CTSEN` writer - CTS flow control"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::DisableAutoCtsFlo)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::EnableAutoCtsFlow)
    }
}
impl R {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrc(&self) -> DtrcR {
        DtrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsc(&self) -> RtscR {
        RtscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS flow control"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS flow control"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn dtrc(&mut self) -> DtrcW<McrSpec> {
        DtrcW::new(self, 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn rtsc(&mut self) -> RtscW<McrSpec> {
        RtscW::new(self, 1)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    #[must_use]
    pub fn lms(&mut self) -> LmsW<McrSpec> {
        LmsW::new(self, 4)
    }
    #[doc = "Bit 6 - RTS flow control"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<McrSpec> {
        RtsenW::new(self, 6)
    }
    #[doc = "Bit 7 - CTS flow control"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<McrSpec> {
        CtsenW::new(self, 7)
    }
}
#[doc = "Modem control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
