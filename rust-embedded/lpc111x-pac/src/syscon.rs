#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sysmemremap: Sysmemremap,
    presetctrl: Presetctrl,
    syspllctrl: Syspllctrl,
    syspllstat: Syspllstat,
    _reserved4: [u8; 0x10],
    sysoscctrl: Sysoscctrl,
    wdtoscctrl: Wdtoscctrl,
    ircctrl: Ircctrl,
    _reserved7: [u8; 0x04],
    sysrststat: Sysrststat,
    _reserved8: [u8; 0x0c],
    syspllclksel: Syspllclksel,
    syspllclkuen: Syspllclkuen,
    _reserved10: [u8; 0x28],
    mainclksel: Mainclksel,
    mainclkuen: Mainclkuen,
    sysahbclkdiv: Sysahbclkdiv,
    _reserved13: [u8; 0x04],
    sysahbclkctrl: Sysahbclkctrl,
    _reserved14: [u8; 0x10],
    ssp0clkdiv: Ssp0clkdiv,
    uartclkdiv: Uartclkdiv,
    ssp1clkdiv: Ssp1clkdiv,
    _reserved17: [u8; 0x30],
    wdtclksel: Wdtclksel,
    wdtclkuen: Wdtclkuen,
    wdtclkdiv: Wdtclkdiv,
    _reserved20: [u8; 0x04],
    clkoutclksel: Clkoutclksel,
    clkoutuen: Clkoutuen,
    clkoutclkdiv: Clkoutclkdiv,
    _reserved23: [u8; 0x14],
    pioporcap0: Pioporcap0,
    pioporcap1: Pioporcap1,
    _reserved25: [u8; 0x48],
    bodctrl: Bodctrl,
    systckcal: Systckcal,
    _reserved27: [u8; 0x1c],
    nmisrc: Nmisrc,
    _reserved28: [u8; 0x88],
    startaprp0: Startaprp0,
    starterp0: Starterp0,
    startrsrp0clr: Startrsrp0clr,
    startsrp0: Startsrp0,
    _reserved32: [u8; 0x20],
    pdsleepcfg: Pdsleepcfg,
    pdawakecfg: Pdawakecfg,
    pdruncfg: Pdruncfg,
    _reserved35: [u8; 0x01b8],
    device_id: DeviceId,
}
impl RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    #[inline(always)]
    pub const fn sysmemremap(&self) -> &Sysmemremap {
        &self.sysmemremap
    }
    #[doc = "0x04 - Peripheral reset control"]
    #[inline(always)]
    pub const fn presetctrl(&self) -> &Presetctrl {
        &self.presetctrl
    }
    #[doc = "0x08 - System PLL control"]
    #[inline(always)]
    pub const fn syspllctrl(&self) -> &Syspllctrl {
        &self.syspllctrl
    }
    #[doc = "0x0c - System PLL status"]
    #[inline(always)]
    pub const fn syspllstat(&self) -> &Syspllstat {
        &self.syspllstat
    }
    #[doc = "0x20 - System oscillator control"]
    #[inline(always)]
    pub const fn sysoscctrl(&self) -> &Sysoscctrl {
        &self.sysoscctrl
    }
    #[doc = "0x24 - Watchdog oscillator control"]
    #[inline(always)]
    pub const fn wdtoscctrl(&self) -> &Wdtoscctrl {
        &self.wdtoscctrl
    }
    #[doc = "0x28 - IRC control"]
    #[inline(always)]
    pub const fn ircctrl(&self) -> &Ircctrl {
        &self.ircctrl
    }
    #[doc = "0x30 - System reset status register"]
    #[inline(always)]
    pub const fn sysrststat(&self) -> &Sysrststat {
        &self.sysrststat
    }
    #[doc = "0x40 - System PLL clock source select"]
    #[inline(always)]
    pub const fn syspllclksel(&self) -> &Syspllclksel {
        &self.syspllclksel
    }
    #[doc = "0x44 - System PLL clock source update enable"]
    #[inline(always)]
    pub const fn syspllclkuen(&self) -> &Syspllclkuen {
        &self.syspllclkuen
    }
    #[doc = "0x70 - Main clock source select"]
    #[inline(always)]
    pub const fn mainclksel(&self) -> &Mainclksel {
        &self.mainclksel
    }
    #[doc = "0x74 - Main clock source update enable"]
    #[inline(always)]
    pub const fn mainclkuen(&self) -> &Mainclkuen {
        &self.mainclkuen
    }
    #[doc = "0x78 - System AHB clock divider"]
    #[inline(always)]
    pub const fn sysahbclkdiv(&self) -> &Sysahbclkdiv {
        &self.sysahbclkdiv
    }
    #[doc = "0x80 - System AHB clock control"]
    #[inline(always)]
    pub const fn sysahbclkctrl(&self) -> &Sysahbclkctrl {
        &self.sysahbclkctrl
    }
    #[doc = "0x94 - SPI0 clock divider"]
    #[inline(always)]
    pub const fn ssp0clkdiv(&self) -> &Ssp0clkdiv {
        &self.ssp0clkdiv
    }
    #[doc = "0x98 - UART clock divder"]
    #[inline(always)]
    pub const fn uartclkdiv(&self) -> &Uartclkdiv {
        &self.uartclkdiv
    }
    #[doc = "0x9c - SPI1 clock divder"]
    #[inline(always)]
    pub const fn ssp1clkdiv(&self) -> &Ssp1clkdiv {
        &self.ssp1clkdiv
    }
    #[doc = "0xd0 - WDT clock source select"]
    #[inline(always)]
    pub const fn wdtclksel(&self) -> &Wdtclksel {
        &self.wdtclksel
    }
    #[doc = "0xd4 - WDT clock source update enable"]
    #[inline(always)]
    pub const fn wdtclkuen(&self) -> &Wdtclkuen {
        &self.wdtclkuen
    }
    #[doc = "0xd8 - WDT clock divider"]
    #[inline(always)]
    pub const fn wdtclkdiv(&self) -> &Wdtclkdiv {
        &self.wdtclkdiv
    }
    #[doc = "0xe0 - CLKOUT clock source select"]
    #[inline(always)]
    pub const fn clkoutclksel(&self) -> &Clkoutclksel {
        &self.clkoutclksel
    }
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    #[inline(always)]
    pub const fn clkoutuen(&self) -> &Clkoutuen {
        &self.clkoutuen
    }
    #[doc = "0xe8 - CLKOUT clock divider"]
    #[inline(always)]
    pub const fn clkoutclkdiv(&self) -> &Clkoutclkdiv {
        &self.clkoutclkdiv
    }
    #[doc = "0x100 - POR captured PIO status 0"]
    #[inline(always)]
    pub const fn pioporcap0(&self) -> &Pioporcap0 {
        &self.pioporcap0
    }
    #[doc = "0x104 - POR captured PIO status 1"]
    #[inline(always)]
    pub const fn pioporcap1(&self) -> &Pioporcap1 {
        &self.pioporcap1
    }
    #[doc = "0x150 - BOD control"]
    #[inline(always)]
    pub const fn bodctrl(&self) -> &Bodctrl {
        &self.bodctrl
    }
    #[doc = "0x154 - System tick counter calibration"]
    #[inline(always)]
    pub const fn systckcal(&self) -> &Systckcal {
        &self.systckcal
    }
    #[doc = "0x174 - NMI source selection"]
    #[inline(always)]
    pub const fn nmisrc(&self) -> &Nmisrc {
        &self.nmisrc
    }
    #[doc = "0x200 - Start logic edge control register 0"]
    #[inline(always)]
    pub const fn startaprp0(&self) -> &Startaprp0 {
        &self.startaprp0
    }
    #[doc = "0x204 - Start logic signal enable register 0"]
    #[inline(always)]
    pub const fn starterp0(&self) -> &Starterp0 {
        &self.starterp0
    }
    #[doc = "0x208 - Start logic reset register 0"]
    #[inline(always)]
    pub const fn startrsrp0clr(&self) -> &Startrsrp0clr {
        &self.startrsrp0clr
    }
    #[doc = "0x20c - Start logic status register 0"]
    #[inline(always)]
    pub const fn startsrp0(&self) -> &Startsrp0 {
        &self.startsrp0
    }
    #[doc = "0x230 - Power-down states in Deep-sleep mode"]
    #[inline(always)]
    pub const fn pdsleepcfg(&self) -> &Pdsleepcfg {
        &self.pdsleepcfg
    }
    #[doc = "0x234 - Power-down states after wake-up from Deep-sleep mode"]
    #[inline(always)]
    pub const fn pdawakecfg(&self) -> &Pdawakecfg {
        &self.pdawakecfg
    }
    #[doc = "0x238 - Power-down configuration register"]
    #[inline(always)]
    pub const fn pdruncfg(&self) -> &Pdruncfg {
        &self.pdruncfg
    }
    #[doc = "0x3f4 - Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L."]
    #[inline(always)]
    pub const fn device_id(&self) -> &DeviceId {
        &self.device_id
    }
}
#[doc = "SYSMEMREMAP (rw) register accessor: System memory remap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysmemremap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysmemremap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmemremap`]
module"]
#[doc(alias = "SYSMEMREMAP")]
pub type Sysmemremap = crate::Reg<sysmemremap::SysmemremapSpec>;
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "PRESETCTRL (rw) register accessor: Peripheral reset control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presetctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presetctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl`]
module"]
#[doc(alias = "PRESETCTRL")]
pub type Presetctrl = crate::Reg<presetctrl::PresetctrlSpec>;
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "SYSPLLCTRL (rw) register accessor: System PLL control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syspllctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllctrl`]
module"]
#[doc(alias = "SYSPLLCTRL")]
pub type Syspllctrl = crate::Reg<syspllctrl::SyspllctrlSpec>;
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT (r) register accessor: System PLL status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllstat`]
module"]
#[doc(alias = "SYSPLLSTAT")]
pub type Syspllstat = crate::Reg<syspllstat::SyspllstatSpec>;
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "SYSOSCCTRL (rw) register accessor: System oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysoscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysoscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysoscctrl`]
module"]
#[doc(alias = "SYSOSCCTRL")]
pub type Sysoscctrl = crate::Reg<sysoscctrl::SysoscctrlSpec>;
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL (rw) register accessor: Watchdog oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtoscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtoscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtoscctrl`]
module"]
#[doc(alias = "WDTOSCCTRL")]
pub type Wdtoscctrl = crate::Reg<wdtoscctrl::WdtoscctrlSpec>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "IRCCTRL (rw) register accessor: IRC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ircctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ircctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ircctrl`]
module"]
#[doc(alias = "IRCCTRL")]
pub type Ircctrl = crate::Reg<ircctrl::IrcctrlSpec>;
#[doc = "IRC control"]
pub mod ircctrl;
#[doc = "SYSRSTSTAT (r) register accessor: System reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrststat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrststat`]
module"]
#[doc(alias = "SYSRSTSTAT")]
pub type Sysrststat = crate::Reg<sysrststat::SysrststatSpec>;
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "SYSPLLCLKSEL (rw) register accessor: System PLL clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syspllclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllclksel`]
module"]
#[doc(alias = "SYSPLLCLKSEL")]
pub type Syspllclksel = crate::Reg<syspllclksel::SyspllclkselSpec>;
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN (rw) register accessor: System PLL clock source update enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syspllclkuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syspllclkuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllclkuen`]
module"]
#[doc(alias = "SYSPLLCLKUEN")]
pub type Syspllclkuen = crate::Reg<syspllclkuen::SyspllclkuenSpec>;
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "MAINCLKSEL (rw) register accessor: Main clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mainclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mainclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclksel`]
module"]
#[doc(alias = "MAINCLKSEL")]
pub type Mainclksel = crate::Reg<mainclksel::MainclkselSpec>;
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "MAINCLKUEN (rw) register accessor: Main clock source update enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mainclkuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mainclkuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclkuen`]
module"]
#[doc(alias = "MAINCLKUEN")]
pub type Mainclkuen = crate::Reg<mainclkuen::MainclkuenSpec>;
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "SYSAHBCLKDIV (rw) register accessor: System AHB clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysahbclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysahbclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysahbclkdiv`]
module"]
#[doc(alias = "SYSAHBCLKDIV")]
pub type Sysahbclkdiv = crate::Reg<sysahbclkdiv::SysahbclkdivSpec>;
#[doc = "System AHB clock divider"]
pub mod sysahbclkdiv;
#[doc = "SYSAHBCLKCTRL (rw) register accessor: System AHB clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysahbclkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysahbclkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysahbclkctrl`]
module"]
#[doc(alias = "SYSAHBCLKCTRL")]
pub type Sysahbclkctrl = crate::Reg<sysahbclkctrl::SysahbclkctrlSpec>;
#[doc = "System AHB clock control"]
pub mod sysahbclkctrl;
#[doc = "SSP0CLKDIV (rw) register accessor: SPI0 clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp0clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp0clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp0clkdiv`]
module"]
#[doc(alias = "SSP0CLKDIV")]
pub type Ssp0clkdiv = crate::Reg<ssp0clkdiv::Ssp0clkdivSpec>;
#[doc = "SPI0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UARTCLKDIV (rw) register accessor: UART clock divder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartclkdiv`]
module"]
#[doc(alias = "UARTCLKDIV")]
pub type Uartclkdiv = crate::Reg<uartclkdiv::UartclkdivSpec>;
#[doc = "UART clock divder"]
pub mod uartclkdiv;
#[doc = "SSP1CLKDIV (rw) register accessor: SPI1 clock divder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp1clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp1clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp1clkdiv`]
module"]
#[doc(alias = "SSP1CLKDIV")]
pub type Ssp1clkdiv = crate::Reg<ssp1clkdiv::Ssp1clkdivSpec>;
#[doc = "SPI1 clock divder"]
pub mod ssp1clkdiv;
#[doc = "WDTCLKSEL (rw) register accessor: WDT clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclksel`]
module"]
#[doc(alias = "WDTCLKSEL")]
pub type Wdtclksel = crate::Reg<wdtclksel::WdtclkselSpec>;
#[doc = "WDT clock source select"]
pub mod wdtclksel;
#[doc = "WDTCLKUEN (rw) register accessor: WDT clock source update enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkuen`]
module"]
#[doc(alias = "WDTCLKUEN")]
pub type Wdtclkuen = crate::Reg<wdtclkuen::WdtclkuenSpec>;
#[doc = "WDT clock source update enable"]
pub mod wdtclkuen;
#[doc = "WDTCLKDIV (rw) register accessor: WDT clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkdiv`]
module"]
#[doc(alias = "WDTCLKDIV")]
pub type Wdtclkdiv = crate::Reg<wdtclkdiv::WdtclkdivSpec>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "CLKOUTCLKSEL (rw) register accessor: CLKOUT clock source select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutclksel`]
module"]
#[doc(alias = "CLKOUTCLKSEL")]
pub type Clkoutclksel = crate::Reg<clkoutclksel::ClkoutclkselSpec>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutclksel;
#[doc = "CLKOUTUEN (rw) register accessor: CLKOUT clock source update enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutuen`]
module"]
#[doc(alias = "CLKOUTUEN")]
pub type Clkoutuen = crate::Reg<clkoutuen::ClkoutuenSpec>;
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUTCLKDIV (rw) register accessor: CLKOUT clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutclkdiv`]
module"]
#[doc(alias = "CLKOUTCLKDIV")]
pub type Clkoutclkdiv = crate::Reg<clkoutclkdiv::ClkoutclkdivSpec>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutclkdiv;
#[doc = "PIOPORCAP0 (r) register accessor: POR captured PIO status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioporcap0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pioporcap0`]
module"]
#[doc(alias = "PIOPORCAP0")]
pub type Pioporcap0 = crate::Reg<pioporcap0::Pioporcap0Spec>;
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "PIOPORCAP1 (r) register accessor: POR captured PIO status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioporcap1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pioporcap1`]
module"]
#[doc(alias = "PIOPORCAP1")]
pub type Pioporcap1 = crate::Reg<pioporcap1::Pioporcap1Spec>;
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BODCTRL (rw) register accessor: BOD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodctrl`]
module"]
#[doc(alias = "BODCTRL")]
pub type Bodctrl = crate::Reg<bodctrl::BodctrlSpec>;
#[doc = "BOD control"]
pub mod bodctrl;
#[doc = "SYSTCKCAL (rw) register accessor: System tick counter calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systckcal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systckcal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systckcal`]
module"]
#[doc(alias = "SYSTCKCAL")]
pub type Systckcal = crate::Reg<systckcal::SystckcalSpec>;
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "NMISRC (rw) register accessor: NMI source selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmisrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmisrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisrc`]
module"]
#[doc(alias = "NMISRC")]
pub type Nmisrc = crate::Reg<nmisrc::NmisrcSpec>;
#[doc = "NMI source selection"]
pub mod nmisrc;
#[doc = "STARTAPRP0 (rw) register accessor: Start logic edge control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startaprp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startaprp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startaprp0`]
module"]
#[doc(alias = "STARTAPRP0")]
pub type Startaprp0 = crate::Reg<startaprp0::Startaprp0Spec>;
#[doc = "Start logic edge control register 0"]
pub mod startaprp0;
#[doc = "STARTERP0 (rw) register accessor: Start logic signal enable register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`starterp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`starterp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starterp0`]
module"]
#[doc(alias = "STARTERP0")]
pub type Starterp0 = crate::Reg<starterp0::Starterp0Spec>;
#[doc = "Start logic signal enable register 0"]
pub mod starterp0;
#[doc = "STARTRSRP0CLR (w) register accessor: Start logic reset register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startrsrp0clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startrsrp0clr`]
module"]
#[doc(alias = "STARTRSRP0CLR")]
pub type Startrsrp0clr = crate::Reg<startrsrp0clr::Startrsrp0clrSpec>;
#[doc = "Start logic reset register 0"]
pub mod startrsrp0clr;
#[doc = "STARTSRP0 (r) register accessor: Start logic status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startsrp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startsrp0`]
module"]
#[doc(alias = "STARTSRP0")]
pub type Startsrp0 = crate::Reg<startsrp0::Startsrp0Spec>;
#[doc = "Start logic status register 0"]
pub mod startsrp0;
#[doc = "PDSLEEPCFG (rw) register accessor: Power-down states in Deep-sleep mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdsleepcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdsleepcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsleepcfg`]
module"]
#[doc(alias = "PDSLEEPCFG")]
pub type Pdsleepcfg = crate::Reg<pdsleepcfg::PdsleepcfgSpec>;
#[doc = "Power-down states in Deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG (rw) register accessor: Power-down states after wake-up from Deep-sleep mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdawakecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdawakecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdawakecfg`]
module"]
#[doc(alias = "PDAWAKECFG")]
pub type Pdawakecfg = crate::Reg<pdawakecfg::PdawakecfgSpec>;
#[doc = "Power-down states after wake-up from Deep-sleep mode"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG (rw) register accessor: Power-down configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdruncfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdruncfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg`]
module"]
#[doc(alias = "PDRUNCFG")]
pub type Pdruncfg = crate::Reg<pdruncfg::PdruncfgSpec>;
#[doc = "Power-down configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID (r) register accessor: Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id`]
module"]
#[doc(alias = "DEVICE_ID")]
pub type DeviceId = crate::Reg<device_id::DeviceIdSpec>;
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L."]
pub mod device_id;
