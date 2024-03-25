#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pio2_6: Pio2_6,
    _reserved1: [u8; 0x04],
    pio2_0: Pio2_0,
    reset_pio0_0: ResetPio0_0,
    pio0_1: Pio0_1,
    pio1_8: Pio1_8,
    ssel1_loc: Ssel1Loc,
    pio0_2: Pio0_2,
    pio2_7: Pio2_7,
    pio2_8: Pio2_8,
    pio2_1: Pio2_1,
    pio0_3: Pio0_3,
    pio0_4: Pio0_4,
    pio0_5: Pio0_5,
    pio1_9: Pio1_9,
    pio3_4: Pio3_4,
    pio2_4: Pio2_4,
    pio2_5: Pio2_5,
    pio3_5: Pio3_5,
    pio0_6: Pio0_6,
    pio0_7: Pio0_7,
    pio2_9: Pio2_9,
    pio2_10: Pio2_10,
    pio2_2: Pio2_2,
    pio0_8: Pio0_8,
    pio0_9: Pio0_9,
    swclk_pio0_10: SwclkPio0_10,
    pio1_10: Pio1_10,
    pio2_11: Pio2_11,
    r_pio0_11: RPio0_11,
    r_pio1_0: RPio1_0,
    r_pio1_1: RPio1_1,
    r_pio1_2: RPio1_2,
    pio3_0: Pio3_0,
    pio3_1: Pio3_1,
    pio2_3: Pio2_3,
    swdio_pio1_3: SwdioPio1_3,
    pio1_4: Pio1_4,
    pio1_11: Pio1_11,
    pio3_2: Pio3_2,
    pio1_5: Pio1_5,
    pio1_6: Pio1_6,
    pio1_7: Pio1_7,
    pio3_3: Pio3_3,
    sck0_loc: Sck0Loc,
    dsr_loc: DsrLoc,
    dcd_loc: DcdLoc,
    ri_loc: RiLoc,
    ct16b0_cap0_loc: Ct16b0Cap0Loc,
    sck1_loc: Sck1Loc,
    miso1_loc: Miso1Loc,
    mosi1_loc: Mosi1Loc,
    ct32b0_cap0_loc: Ct32b0Cap0Loc,
    rxd_loc: RxdLoc,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
    #[inline(always)]
    pub const fn pio2_6(&self) -> &Pio2_6 {
        &self.pio2_6
    }
    #[doc = "0x08 - I/O configuration for pin PIO2_0/DTR/SSEL1"]
    #[inline(always)]
    pub const fn pio2_0(&self) -> &Pio2_0 {
        &self.pio2_0
    }
    #[doc = "0x0c - I/O configuration for pin RESET/PIO0_0"]
    #[inline(always)]
    pub const fn reset_pio0_0(&self) -> &ResetPio0_0 {
        &self.reset_pio0_0
    }
    #[doc = "0x10 - I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
    #[inline(always)]
    pub const fn pio0_1(&self) -> &Pio0_1 {
        &self.pio0_1
    }
    #[doc = "0x14 - I/O configuration for pin PIO1_8/CT16B1_CAP0"]
    #[inline(always)]
    pub const fn pio1_8(&self) -> &Pio1_8 {
        &self.pio1_8
    }
    #[doc = "0x18 - SSEL1 pin location select register"]
    #[inline(always)]
    pub const fn ssel1_loc(&self) -> &Ssel1Loc {
        &self.ssel1_loc
    }
    #[doc = "0x1c - I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
    #[inline(always)]
    pub const fn pio0_2(&self) -> &Pio0_2 {
        &self.pio0_2
    }
    #[doc = "0x20 - I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
    #[inline(always)]
    pub const fn pio2_7(&self) -> &Pio2_7 {
        &self.pio2_7
    }
    #[doc = "0x24 - I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
    #[inline(always)]
    pub const fn pio2_8(&self) -> &Pio2_8 {
        &self.pio2_8
    }
    #[doc = "0x28 - I/O configuration for pin PIO2_1/DSR/SCK1"]
    #[inline(always)]
    pub const fn pio2_1(&self) -> &Pio2_1 {
        &self.pio2_1
    }
    #[doc = "0x2c - I/O configuration for pin PIO0_3"]
    #[inline(always)]
    pub const fn pio0_3(&self) -> &Pio0_3 {
        &self.pio0_3
    }
    #[doc = "0x30 - I/O configuration for pin PIO0_4/SCL"]
    #[inline(always)]
    pub const fn pio0_4(&self) -> &Pio0_4 {
        &self.pio0_4
    }
    #[doc = "0x34 - I/O configuration for pin PIO0_5/SDA"]
    #[inline(always)]
    pub const fn pio0_5(&self) -> &Pio0_5 {
        &self.pio0_5
    }
    #[doc = "0x38 - I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
    #[inline(always)]
    pub const fn pio1_9(&self) -> &Pio1_9 {
        &self.pio1_9
    }
    #[doc = "0x3c - I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
    #[inline(always)]
    pub const fn pio3_4(&self) -> &Pio3_4 {
        &self.pio3_4
    }
    #[doc = "0x40 - I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
    #[inline(always)]
    pub const fn pio2_4(&self) -> &Pio2_4 {
        &self.pio2_4
    }
    #[doc = "0x44 - I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
    #[inline(always)]
    pub const fn pio2_5(&self) -> &Pio2_5 {
        &self.pio2_5
    }
    #[doc = "0x48 - I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
    #[inline(always)]
    pub const fn pio3_5(&self) -> &Pio3_5 {
        &self.pio3_5
    }
    #[doc = "0x4c - I/O configuration for pin PIO0_6/SCK0"]
    #[inline(always)]
    pub const fn pio0_6(&self) -> &Pio0_6 {
        &self.pio0_6
    }
    #[doc = "0x50 - I/O configuration for pin PIO0_7/CTS"]
    #[inline(always)]
    pub const fn pio0_7(&self) -> &Pio0_7 {
        &self.pio0_7
    }
    #[doc = "0x54 - I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
    #[inline(always)]
    pub const fn pio2_9(&self) -> &Pio2_9 {
        &self.pio2_9
    }
    #[doc = "0x58 - I/O configuration for pin PIO2_10"]
    #[inline(always)]
    pub const fn pio2_10(&self) -> &Pio2_10 {
        &self.pio2_10
    }
    #[doc = "0x5c - I/O configuration for pin PIO2_2/DCD/MISO1"]
    #[inline(always)]
    pub const fn pio2_2(&self) -> &Pio2_2 {
        &self.pio2_2
    }
    #[doc = "0x60 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    #[inline(always)]
    pub const fn pio0_8(&self) -> &Pio0_8 {
        &self.pio0_8
    }
    #[doc = "0x64 - I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
    #[inline(always)]
    pub const fn pio0_9(&self) -> &Pio0_9 {
        &self.pio0_9
    }
    #[doc = "0x68 - I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
    #[inline(always)]
    pub const fn swclk_pio0_10(&self) -> &SwclkPio0_10 {
        &self.swclk_pio0_10
    }
    #[doc = "0x6c - I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
    #[inline(always)]
    pub const fn pio1_10(&self) -> &Pio1_10 {
        &self.pio1_10
    }
    #[doc = "0x70 - I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
    #[inline(always)]
    pub const fn pio2_11(&self) -> &Pio2_11 {
        &self.pio2_11
    }
    #[doc = "0x74 - I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
    #[inline(always)]
    pub const fn r_pio0_11(&self) -> &RPio0_11 {
        &self.r_pio0_11
    }
    #[doc = "0x78 - I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
    #[inline(always)]
    pub const fn r_pio1_0(&self) -> &RPio1_0 {
        &self.r_pio1_0
    }
    #[doc = "0x7c - I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
    #[inline(always)]
    pub const fn r_pio1_1(&self) -> &RPio1_1 {
        &self.r_pio1_1
    }
    #[doc = "0x80 - I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
    #[inline(always)]
    pub const fn r_pio1_2(&self) -> &RPio1_2 {
        &self.r_pio1_2
    }
    #[doc = "0x84 - I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
    #[inline(always)]
    pub const fn pio3_0(&self) -> &Pio3_0 {
        &self.pio3_0
    }
    #[doc = "0x88 - I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
    #[inline(always)]
    pub const fn pio3_1(&self) -> &Pio3_1 {
        &self.pio3_1
    }
    #[doc = "0x8c - I/O configuration for pin PIO2_3/RI/MOSI1"]
    #[inline(always)]
    pub const fn pio2_3(&self) -> &Pio2_3 {
        &self.pio2_3
    }
    #[doc = "0x90 - I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
    #[inline(always)]
    pub const fn swdio_pio1_3(&self) -> &SwdioPio1_3 {
        &self.swdio_pio1_3
    }
    #[doc = "0x94 - I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
    #[inline(always)]
    pub const fn pio1_4(&self) -> &Pio1_4 {
        &self.pio1_4
    }
    #[doc = "0x98 - I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
    #[inline(always)]
    pub const fn pio1_11(&self) -> &Pio1_11 {
        &self.pio1_11
    }
    #[doc = "0x9c - I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
    #[inline(always)]
    pub const fn pio3_2(&self) -> &Pio3_2 {
        &self.pio3_2
    }
    #[doc = "0xa0 - I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
    #[inline(always)]
    pub const fn pio1_5(&self) -> &Pio1_5 {
        &self.pio1_5
    }
    #[doc = "0xa4 - I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
    #[inline(always)]
    pub const fn pio1_6(&self) -> &Pio1_6 {
        &self.pio1_6
    }
    #[doc = "0xa8 - I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
    #[inline(always)]
    pub const fn pio1_7(&self) -> &Pio1_7 {
        &self.pio1_7
    }
    #[doc = "0xac - I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
    #[inline(always)]
    pub const fn pio3_3(&self) -> &Pio3_3 {
        &self.pio3_3
    }
    #[doc = "0xb0 - SCK0 pin location select register"]
    #[inline(always)]
    pub const fn sck0_loc(&self) -> &Sck0Loc {
        &self.sck0_loc
    }
    #[doc = "0xb4 - DSR pin location select register"]
    #[inline(always)]
    pub const fn dsr_loc(&self) -> &DsrLoc {
        &self.dsr_loc
    }
    #[doc = "0xb8 - DCD pin location select register"]
    #[inline(always)]
    pub const fn dcd_loc(&self) -> &DcdLoc {
        &self.dcd_loc
    }
    #[doc = "0xbc - RI pin location select register"]
    #[inline(always)]
    pub const fn ri_loc(&self) -> &RiLoc {
        &self.ri_loc
    }
    #[doc = "0xc0 - CT16B0_CAP0 pin location select register"]
    #[inline(always)]
    pub const fn ct16b0_cap0_loc(&self) -> &Ct16b0Cap0Loc {
        &self.ct16b0_cap0_loc
    }
    #[doc = "0xc4 - SCK1 pin location select register"]
    #[inline(always)]
    pub const fn sck1_loc(&self) -> &Sck1Loc {
        &self.sck1_loc
    }
    #[doc = "0xc8 - MISO1 pin location select register"]
    #[inline(always)]
    pub const fn miso1_loc(&self) -> &Miso1Loc {
        &self.miso1_loc
    }
    #[doc = "0xcc - MOSI1 pin location select register"]
    #[inline(always)]
    pub const fn mosi1_loc(&self) -> &Mosi1Loc {
        &self.mosi1_loc
    }
    #[doc = "0xd0 - CT32B0_CAP0 pin location select register"]
    #[inline(always)]
    pub const fn ct32b0_cap0_loc(&self) -> &Ct32b0Cap0Loc {
        &self.ct32b0_cap0_loc
    }
    #[doc = "0xd4 - RXD pin location select register"]
    #[inline(always)]
    pub const fn rxd_loc(&self) -> &RxdLoc {
        &self.rxd_loc
    }
}
#[doc = "PIO2_6 (rw) register accessor: I/O configuration for pin PIO2_6/ CT32B0_MAT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_6`]
module"]
#[doc(alias = "PIO2_6")]
pub type Pio2_6 = crate::Reg<pio2_6::Pio2_6Spec>;
#[doc = "I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
pub mod pio2_6;
#[doc = "PIO2_0 (rw) register accessor: I/O configuration for pin PIO2_0/DTR/SSEL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_0`]
module"]
#[doc(alias = "PIO2_0")]
pub type Pio2_0 = crate::Reg<pio2_0::Pio2_0Spec>;
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub mod pio2_0;
#[doc = "RESET_PIO0_0 (rw) register accessor: I/O configuration for pin RESET/PIO0_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_pio0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_pio0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_pio0_0`]
module"]
#[doc(alias = "RESET_PIO0_0")]
pub type ResetPio0_0 = crate::Reg<reset_pio0_0::ResetPio0_0Spec>;
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "PIO0_1 (rw) register accessor: I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_1`]
module"]
#[doc(alias = "PIO0_1")]
pub type Pio0_1 = crate::Reg<pio0_1::Pio0_1Spec>;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
pub mod pio0_1;
#[doc = "PIO1_8 (rw) register accessor: I/O configuration for pin PIO1_8/CT16B1_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_8`]
module"]
#[doc(alias = "PIO1_8")]
pub type Pio1_8 = crate::Reg<pio1_8::Pio1_8Spec>;
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub mod pio1_8;
#[doc = "PIO0_2 (rw) register accessor: I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_2`]
module"]
#[doc(alias = "PIO0_2")]
pub type Pio0_2 = crate::Reg<pio0_2::Pio0_2Spec>;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "PIO2_7 (rw) register accessor: I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_7`]
module"]
#[doc(alias = "PIO2_7")]
pub type Pio2_7 = crate::Reg<pio2_7::Pio2_7Spec>;
#[doc = "I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
pub mod pio2_7;
#[doc = "PIO2_8 (rw) register accessor: I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_8`]
module"]
#[doc(alias = "PIO2_8")]
pub type Pio2_8 = crate::Reg<pio2_8::Pio2_8Spec>;
#[doc = "I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
pub mod pio2_8;
#[doc = "PIO2_1 (rw) register accessor: I/O configuration for pin PIO2_1/DSR/SCK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_1`]
module"]
#[doc(alias = "PIO2_1")]
pub type Pio2_1 = crate::Reg<pio2_1::Pio2_1Spec>;
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub mod pio2_1;
#[doc = "PIO0_3 (rw) register accessor: I/O configuration for pin PIO0_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_3`]
module"]
#[doc(alias = "PIO0_3")]
pub type Pio0_3 = crate::Reg<pio0_3::Pio0_3Spec>;
#[doc = "I/O configuration for pin PIO0_3"]
pub mod pio0_3;
#[doc = "PIO0_4 (rw) register accessor: I/O configuration for pin PIO0_4/SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_4`]
module"]
#[doc(alias = "PIO0_4")]
pub type Pio0_4 = crate::Reg<pio0_4::Pio0_4Spec>;
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "PIO0_5 (rw) register accessor: I/O configuration for pin PIO0_5/SDA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_5`]
module"]
#[doc(alias = "PIO0_5")]
pub type Pio0_5 = crate::Reg<pio0_5::Pio0_5Spec>;
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "PIO1_9 (rw) register accessor: I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_9`]
module"]
#[doc(alias = "PIO1_9")]
pub type Pio1_9 = crate::Reg<pio1_9::Pio1_9Spec>;
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
pub mod pio1_9;
#[doc = "PIO3_4 (rw) register accessor: I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_4`]
module"]
#[doc(alias = "PIO3_4")]
pub type Pio3_4 = crate::Reg<pio3_4::Pio3_4Spec>;
#[doc = "I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
pub mod pio3_4;
#[doc = "PIO2_4 (rw) register accessor: I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_4`]
module"]
#[doc(alias = "PIO2_4")]
pub type Pio2_4 = crate::Reg<pio2_4::Pio2_4Spec>;
#[doc = "I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
pub mod pio2_4;
#[doc = "PIO2_5 (rw) register accessor: I/O configuration for pin PIO2_5/ CT32B0_MAT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_5`]
module"]
#[doc(alias = "PIO2_5")]
pub type Pio2_5 = crate::Reg<pio2_5::Pio2_5Spec>;
#[doc = "I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
pub mod pio2_5;
#[doc = "PIO3_5 (rw) register accessor: I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_5`]
module"]
#[doc(alias = "PIO3_5")]
pub type Pio3_5 = crate::Reg<pio3_5::Pio3_5Spec>;
#[doc = "I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
pub mod pio3_5;
#[doc = "PIO0_6 (rw) register accessor: I/O configuration for pin PIO0_6/SCK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_6`]
module"]
#[doc(alias = "PIO0_6")]
pub type Pio0_6 = crate::Reg<pio0_6::Pio0_6Spec>;
#[doc = "I/O configuration for pin PIO0_6/SCK0"]
pub mod pio0_6;
#[doc = "PIO0_7 (rw) register accessor: I/O configuration for pin PIO0_7/CTS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_7`]
module"]
#[doc(alias = "PIO0_7")]
pub type Pio0_7 = crate::Reg<pio0_7::Pio0_7Spec>;
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "PIO2_9 (rw) register accessor: I/O configuration for pin PIO2_9/ CT32B0_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_9`]
module"]
#[doc(alias = "PIO2_9")]
pub type Pio2_9 = crate::Reg<pio2_9::Pio2_9Spec>;
#[doc = "I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
pub mod pio2_9;
#[doc = "PIO2_10 (rw) register accessor: I/O configuration for pin PIO2_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_10`]
module"]
#[doc(alias = "PIO2_10")]
pub type Pio2_10 = crate::Reg<pio2_10::Pio2_10Spec>;
#[doc = "I/O configuration for pin PIO2_10"]
pub mod pio2_10;
#[doc = "PIO2_2 (rw) register accessor: I/O configuration for pin PIO2_2/DCD/MISO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_2`]
module"]
#[doc(alias = "PIO2_2")]
pub type Pio2_2 = crate::Reg<pio2_2::Pio2_2Spec>;
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub mod pio2_2;
#[doc = "PIO0_8 (rw) register accessor: I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_8`]
module"]
#[doc(alias = "PIO0_8")]
pub type Pio0_8 = crate::Reg<pio0_8::Pio0_8Spec>;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "PIO0_9 (rw) register accessor: I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio0_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio0_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_9`]
module"]
#[doc(alias = "PIO0_9")]
pub type Pio0_9 = crate::Reg<pio0_9::Pio0_9Spec>;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod pio0_9;
#[doc = "SWCLK_PIO0_10 (rw) register accessor: I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swclk_pio0_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swclk_pio0_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swclk_pio0_10`]
module"]
#[doc(alias = "SWCLK_PIO0_10")]
pub type SwclkPio0_10 = crate::Reg<swclk_pio0_10::SwclkPio0_10Spec>;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "PIO1_10 (rw) register accessor: I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_10`]
module"]
#[doc(alias = "PIO1_10")]
pub type Pio1_10 = crate::Reg<pio1_10::Pio1_10Spec>;
#[doc = "I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
pub mod pio1_10;
#[doc = "PIO2_11 (rw) register accessor: I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_11`]
module"]
#[doc(alias = "PIO2_11")]
pub type Pio2_11 = crate::Reg<pio2_11::Pio2_11Spec>;
#[doc = "I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
pub mod pio2_11;
#[doc = "R_PIO0_11 (rw) register accessor: I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r_pio0_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_pio0_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_pio0_11`]
module"]
#[doc(alias = "R_PIO0_11")]
pub type RPio0_11 = crate::Reg<r_pio0_11::RPio0_11Spec>;
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub mod r_pio0_11;
#[doc = "R_PIO1_0 (rw) register accessor: I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r_pio1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_pio1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_pio1_0`]
module"]
#[doc(alias = "R_PIO1_0")]
pub type RPio1_0 = crate::Reg<r_pio1_0::RPio1_0Spec>;
#[doc = "I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
pub mod r_pio1_0;
#[doc = "R_PIO1_1 (rw) register accessor: I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r_pio1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_pio1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_pio1_1`]
module"]
#[doc(alias = "R_PIO1_1")]
pub type RPio1_1 = crate::Reg<r_pio1_1::RPio1_1Spec>;
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub mod r_pio1_1;
#[doc = "R_PIO1_2 (rw) register accessor: I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r_pio1_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_pio1_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_pio1_2`]
module"]
#[doc(alias = "R_PIO1_2")]
pub type RPio1_2 = crate::Reg<r_pio1_2::RPio1_2Spec>;
#[doc = "I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
pub mod r_pio1_2;
#[doc = "PIO3_0 (rw) register accessor: I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_0`]
module"]
#[doc(alias = "PIO3_0")]
pub type Pio3_0 = crate::Reg<pio3_0::Pio3_0Spec>;
#[doc = "I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
pub mod pio3_0;
#[doc = "PIO3_1 (rw) register accessor: I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_1`]
module"]
#[doc(alias = "PIO3_1")]
pub type Pio3_1 = crate::Reg<pio3_1::Pio3_1Spec>;
#[doc = "I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
pub mod pio3_1;
#[doc = "PIO2_3 (rw) register accessor: I/O configuration for pin PIO2_3/RI/MOSI1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio2_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio2_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_3`]
module"]
#[doc(alias = "PIO2_3")]
pub type Pio2_3 = crate::Reg<pio2_3::Pio2_3Spec>;
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub mod pio2_3;
#[doc = "SWDIO_PIO1_3 (rw) register accessor: I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swdio_pio1_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swdio_pio1_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdio_pio1_3`]
module"]
#[doc(alias = "SWDIO_PIO1_3")]
pub type SwdioPio1_3 = crate::Reg<swdio_pio1_3::SwdioPio1_3Spec>;
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
pub mod swdio_pio1_3;
#[doc = "PIO1_4 (rw) register accessor: I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_4`]
module"]
#[doc(alias = "PIO1_4")]
pub type Pio1_4 = crate::Reg<pio1_4::Pio1_4Spec>;
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub mod pio1_4;
#[doc = "PIO1_11 (rw) register accessor: I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_11`]
module"]
#[doc(alias = "PIO1_11")]
pub type Pio1_11 = crate::Reg<pio1_11::Pio1_11Spec>;
#[doc = "I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
pub mod pio1_11;
#[doc = "PIO3_2 (rw) register accessor: I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_2`]
module"]
#[doc(alias = "PIO3_2")]
pub type Pio3_2 = crate::Reg<pio3_2::Pio3_2Spec>;
#[doc = "I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
pub mod pio3_2;
#[doc = "PIO1_5 (rw) register accessor: I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_5`]
module"]
#[doc(alias = "PIO1_5")]
pub type Pio1_5 = crate::Reg<pio1_5::Pio1_5Spec>;
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub mod pio1_5;
#[doc = "PIO1_6 (rw) register accessor: I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_6`]
module"]
#[doc(alias = "PIO1_6")]
pub type Pio1_6 = crate::Reg<pio1_6::Pio1_6Spec>;
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub mod pio1_6;
#[doc = "PIO1_7 (rw) register accessor: I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio1_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio1_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_7`]
module"]
#[doc(alias = "PIO1_7")]
pub type Pio1_7 = crate::Reg<pio1_7::Pio1_7Spec>;
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub mod pio1_7;
#[doc = "PIO3_3 (rw) register accessor: I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio3_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio3_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_3`]
module"]
#[doc(alias = "PIO3_3")]
pub type Pio3_3 = crate::Reg<pio3_3::Pio3_3Spec>;
#[doc = "I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
pub mod pio3_3;
#[doc = "SCK0_LOC (rw) register accessor: SCK0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sck0_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sck0_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck0_loc`]
module"]
#[doc(alias = "SCK0_LOC")]
pub type Sck0Loc = crate::Reg<sck0_loc::Sck0LocSpec>;
#[doc = "SCK0 pin location select register"]
pub mod sck0_loc;
#[doc = "DSR_LOC (rw) register accessor: DSR pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsr_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_loc`]
module"]
#[doc(alias = "DSR_LOC")]
pub type DsrLoc = crate::Reg<dsr_loc::DsrLocSpec>;
#[doc = "DSR pin location select register"]
pub mod dsr_loc;
#[doc = "DCD_LOC (rw) register accessor: DCD pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcd_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcd_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcd_loc`]
module"]
#[doc(alias = "DCD_LOC")]
pub type DcdLoc = crate::Reg<dcd_loc::DcdLocSpec>;
#[doc = "DCD pin location select register"]
pub mod dcd_loc;
#[doc = "RI_LOC (rw) register accessor: RI pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ri_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ri_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ri_loc`]
module"]
#[doc(alias = "RI_LOC")]
pub type RiLoc = crate::Reg<ri_loc::RiLocSpec>;
#[doc = "RI pin location select register"]
pub mod ri_loc;
#[doc = "SSEL1_LOC (rw) register accessor: SSEL1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssel1_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssel1_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssel1_loc`]
module"]
#[doc(alias = "SSEL1_LOC")]
pub type Ssel1Loc = crate::Reg<ssel1_loc::Ssel1LocSpec>;
#[doc = "SSEL1 pin location select register"]
pub mod ssel1_loc;
#[doc = "CT16B0_CAP0_LOC (rw) register accessor: CT16B0_CAP0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct16b0_cap0_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct16b0_cap0_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct16b0_cap0_loc`]
module"]
#[doc(alias = "CT16B0_CAP0_LOC")]
pub type Ct16b0Cap0Loc = crate::Reg<ct16b0_cap0_loc::Ct16b0Cap0LocSpec>;
#[doc = "CT16B0_CAP0 pin location select register"]
pub mod ct16b0_cap0_loc;
#[doc = "SCK1_LOC (rw) register accessor: SCK1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sck1_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sck1_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck1_loc`]
module"]
#[doc(alias = "SCK1_LOC")]
pub type Sck1Loc = crate::Reg<sck1_loc::Sck1LocSpec>;
#[doc = "SCK1 pin location select register"]
pub mod sck1_loc;
#[doc = "MISO1_LOC (rw) register accessor: MISO1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso1_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso1_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso1_loc`]
module"]
#[doc(alias = "MISO1_LOC")]
pub type Miso1Loc = crate::Reg<miso1_loc::Miso1LocSpec>;
#[doc = "MISO1 pin location select register"]
pub mod miso1_loc;
#[doc = "MOSI1_LOC (rw) register accessor: MOSI1 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosi1_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi1_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi1_loc`]
module"]
#[doc(alias = "MOSI1_LOC")]
pub type Mosi1Loc = crate::Reg<mosi1_loc::Mosi1LocSpec>;
#[doc = "MOSI1 pin location select register"]
pub mod mosi1_loc;
#[doc = "CT32B0_CAP0_LOC (rw) register accessor: CT32B0_CAP0 pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct32b0_cap0_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct32b0_cap0_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct32b0_cap0_loc`]
module"]
#[doc(alias = "CT32B0_CAP0_LOC")]
pub type Ct32b0Cap0Loc = crate::Reg<ct32b0_cap0_loc::Ct32b0Cap0LocSpec>;
#[doc = "CT32B0_CAP0 pin location select register"]
pub mod ct32b0_cap0_loc;
#[doc = "RXD_LOC (rw) register accessor: RXD pin location select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd_loc`]
module"]
#[doc(alias = "RXD_LOC")]
pub type RxdLoc = crate::Reg<rxd_loc::RxdLocSpec>;
#[doc = "RXD pin location select register"]
pub mod rxd_loc;
