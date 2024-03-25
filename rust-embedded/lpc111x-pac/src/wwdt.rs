#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdmod: Wdmod,
    wdtc: Wdtc,
    wdfeed: Wdfeed,
    wdtv: Wdtv,
    _reserved4: [u8; 0x04],
    wdwarnint: Wdwarnint,
    wdwindow: Wdwindow,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    #[inline(always)]
    pub const fn wdmod(&self) -> &Wdmod {
        &self.wdmod
    }
    #[doc = "0x04 - Watchdog timer constant register. This register determines the time-out value."]
    #[inline(always)]
    pub const fn wdtc(&self) -> &Wdtc {
        &self.wdtc
    }
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
    #[inline(always)]
    pub const fn wdfeed(&self) -> &Wdfeed {
        &self.wdfeed
    }
    #[doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
    #[inline(always)]
    pub const fn wdtv(&self) -> &Wdtv {
        &self.wdtv
    }
    #[doc = "0x14 - Watchdog Warning Interrupt compare value."]
    #[inline(always)]
    pub const fn wdwarnint(&self) -> &Wdwarnint {
        &self.wdwarnint
    }
    #[doc = "0x18 - Watchdog Window compare value."]
    #[inline(always)]
    pub const fn wdwindow(&self) -> &Wdwindow {
        &self.wdwindow
    }
}
#[doc = "WDMOD (rw) register accessor: Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdmod`]
module"]
#[doc(alias = "WDMOD")]
pub type Wdmod = crate::Reg<wdmod::WdmodSpec>;
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub mod wdmod;
#[doc = "WDTC (rw) register accessor: Watchdog timer constant register. This register determines the time-out value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtc`]
module"]
#[doc(alias = "WDTC")]
pub type Wdtc = crate::Reg<wdtc::WdtcSpec>;
#[doc = "Watchdog timer constant register. This register determines the time-out value."]
pub mod wdtc;
#[doc = "WDFEED (w) register accessor: Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdfeed`]
module"]
#[doc(alias = "WDFEED")]
pub type Wdfeed = crate::Reg<wdfeed::WdfeedSpec>;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub mod wdfeed;
#[doc = "WDTV (r) register accessor: Watchdog timer value register. This register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtv`]
module"]
#[doc(alias = "WDTV")]
pub type Wdtv = crate::Reg<wdtv::WdtvSpec>;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
pub mod wdtv;
#[doc = "WDWARNINT (rw) register accessor: Watchdog Warning Interrupt compare value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdwarnint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdwarnint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdwarnint`]
module"]
#[doc(alias = "WDWARNINT")]
pub type Wdwarnint = crate::Reg<wdwarnint::WdwarnintSpec>;
#[doc = "Watchdog Warning Interrupt compare value."]
pub mod wdwarnint;
#[doc = "WDWINDOW (rw) register accessor: Watchdog Window compare value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdwindow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdwindow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdwindow`]
module"]
#[doc(alias = "WDWINDOW")]
pub type Wdwindow = crate::Reg<wdwindow::WdwindowSpec>;
#[doc = "Watchdog Window compare value."]
pub mod wdwindow;
