#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    flashcfg: Flashcfg,
    _reserved1: [u8; 0x0c],
    fmsstart: Fmsstart,
    fmsstop: Fmsstop,
    _reserved3: [u8; 0x04],
    fmsw0: Fmsw0,
    fmsw1: Fmsw1,
    fmsw2: Fmsw2,
    fmsw3: Fmsw3,
    _reserved7: [u8; 0x0fa4],
    fmstat: Fmstat,
    _reserved8: [u8; 0x04],
    fmstatclr: Fmstatclr,
}
impl RegisterBlock {
    #[doc = "0x10 - Flash memory access time configuration register"]
    #[inline(always)]
    pub const fn flashcfg(&self) -> &Flashcfg {
        &self.flashcfg
    }
    #[doc = "0x20 - Signature start address register"]
    #[inline(always)]
    pub const fn fmsstart(&self) -> &Fmsstart {
        &self.fmsstart
    }
    #[doc = "0x24 - Signature stop-address register"]
    #[inline(always)]
    pub const fn fmsstop(&self) -> &Fmsstop {
        &self.fmsstop
    }
    #[doc = "0x2c - Word 0 \\[31:0\\]"]
    #[inline(always)]
    pub const fn fmsw0(&self) -> &Fmsw0 {
        &self.fmsw0
    }
    #[doc = "0x30 - Word 1 \\[63:32\\]"]
    #[inline(always)]
    pub const fn fmsw1(&self) -> &Fmsw1 {
        &self.fmsw1
    }
    #[doc = "0x34 - Word 2 \\[95:64\\]"]
    #[inline(always)]
    pub const fn fmsw2(&self) -> &Fmsw2 {
        &self.fmsw2
    }
    #[doc = "0x38 - Word 3 \\[127:96\\]"]
    #[inline(always)]
    pub const fn fmsw3(&self) -> &Fmsw3 {
        &self.fmsw3
    }
    #[doc = "0xfe0 - Signature generation status register"]
    #[inline(always)]
    pub const fn fmstat(&self) -> &Fmstat {
        &self.fmstat
    }
    #[doc = "0xfe8 - Signature generation status clear register"]
    #[inline(always)]
    pub const fn fmstatclr(&self) -> &Fmstatclr {
        &self.fmstatclr
    }
}
#[doc = "FLASHCFG (rw) register accessor: Flash memory access time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`]
module"]
#[doc(alias = "FLASHCFG")]
pub type Flashcfg = crate::Reg<flashcfg::FlashcfgSpec>;
#[doc = "Flash memory access time configuration register"]
pub mod flashcfg;
#[doc = "FMSSTART (rw) register accessor: Signature start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsstart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmsstart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsstart`]
module"]
#[doc(alias = "FMSSTART")]
pub type Fmsstart = crate::Reg<fmsstart::FmsstartSpec>;
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP (rw) register accessor: Signature stop-address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmsstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsstop`]
module"]
#[doc(alias = "FMSSTOP")]
pub type Fmsstop = crate::Reg<fmsstop::FmsstopSpec>;
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "FMSW0 (r) register accessor: Word 0 \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsw0`]
module"]
#[doc(alias = "FMSW0")]
pub type Fmsw0 = crate::Reg<fmsw0::Fmsw0Spec>;
#[doc = "Word 0 \\[31:0\\]"]
pub mod fmsw0;
#[doc = "FMSW1 (r) register accessor: Word 1 \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsw1`]
module"]
#[doc(alias = "FMSW1")]
pub type Fmsw1 = crate::Reg<fmsw1::Fmsw1Spec>;
#[doc = "Word 1 \\[63:32\\]"]
pub mod fmsw1;
#[doc = "FMSW2 (r) register accessor: Word 2 \\[95:64\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsw2`]
module"]
#[doc(alias = "FMSW2")]
pub type Fmsw2 = crate::Reg<fmsw2::Fmsw2Spec>;
#[doc = "Word 2 \\[95:64\\]"]
pub mod fmsw2;
#[doc = "FMSW3 (r) register accessor: Word 3 \\[127:96\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmsw3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmsw3`]
module"]
#[doc(alias = "FMSW3")]
pub type Fmsw3 = crate::Reg<fmsw3::Fmsw3Spec>;
#[doc = "Word 3 \\[127:96\\]"]
pub mod fmsw3;
#[doc = "FMSTAT (r) register accessor: Signature generation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmstat`]
module"]
#[doc(alias = "FMSTAT")]
pub type Fmstat = crate::Reg<fmstat::FmstatSpec>;
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "FMSTATCLR (w) register accessor: Signature generation status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmstatclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmstatclr`]
module"]
#[doc(alias = "FMSTATCLR")]
pub type Fmstatclr = crate::Reg<fmstatclr::FmstatclrSpec>;
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
