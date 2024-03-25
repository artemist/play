#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3ffc],
    data: Data,
    _reserved1: [u8; 0x4000],
    dir: Dir,
    is: Is,
    ibe: Ibe,
    iev: Iev,
    ie: Ie,
    ris: Ris,
    mis: Mis,
    ic: Ic,
}
impl RegisterBlock {
    #[doc = "0x3ffc - Port n data register for pins PIOn_0 to PIOn_11"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x8000 - Data direction register for port n"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x8004 - Interrupt sense register for port n"]
    #[inline(always)]
    pub const fn is(&self) -> &Is {
        &self.is
    }
    #[doc = "0x8008 - Interrupt both edges register for port n"]
    #[inline(always)]
    pub const fn ibe(&self) -> &Ibe {
        &self.ibe
    }
    #[doc = "0x800c - Interrupt event register for port n"]
    #[inline(always)]
    pub const fn iev(&self) -> &Iev {
        &self.iev
    }
    #[doc = "0x8010 - Interrupt mask register for port n"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x8014 - Raw interrupt status register for port n"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x8018 - Masked interrupt status register for port n"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x801c - Interrupt clear register for port n"]
    #[inline(always)]
    pub const fn ic(&self) -> &Ic {
        &self.ic
    }
}
#[doc = "DATA (rw) register accessor: Port n data register for pins PIOn_0 to PIOn_11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Port n data register for pins PIOn_0 to PIOn_11"]
pub mod data;
#[doc = "DIR (rw) register accessor: Data direction register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Data direction register for port n"]
pub mod dir;
#[doc = "IS (rw) register accessor: Interrupt sense register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is`]
module"]
#[doc(alias = "IS")]
pub type Is = crate::Reg<is::IsSpec>;
#[doc = "Interrupt sense register for port n"]
pub mod is;
#[doc = "IBE (rw) register accessor: Interrupt both edges register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibe`]
module"]
#[doc(alias = "IBE")]
pub type Ibe = crate::Reg<ibe::IbeSpec>;
#[doc = "Interrupt both edges register for port n"]
pub mod ibe;
#[doc = "IEV (rw) register accessor: Interrupt event register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iev`]
module"]
#[doc(alias = "IEV")]
pub type Iev = crate::Reg<iev::IevSpec>;
#[doc = "Interrupt event register for port n"]
pub mod iev;
#[doc = "IE (rw) register accessor: Interrupt mask register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt mask register for port n"]
pub mod ie;
#[doc = "RIS (r) register accessor: Raw interrupt status register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status register for port n"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status register for port n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status register for port n"]
pub mod mis;
#[doc = "IC (w) register accessor: Interrupt clear register for port n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`]
module"]
#[doc(alias = "IC")]
pub type Ic = crate::Reg<ic::IcSpec>;
#[doc = "Interrupt clear register for port n"]
pub mod ic;
