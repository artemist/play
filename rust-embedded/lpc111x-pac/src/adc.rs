#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    gdr: Gdr,
    _reserved2: [u8; 0x04],
    inten: Inten,
    dr: [Dr; 8],
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - A/D Global Data Register. Contains the result of the most recent A/D conversion."]
    #[inline(always)]
    pub const fn gdr(&self) -> &Gdr {
        &self.gdr
    }
    #[doc = "0x0c - A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x10..0x30 - A/D Channel n Data Register. This register contains the result of the most recent conversion completed on channel n."]
    #[inline(always)]
    pub const fn dr(&self, n: usize) -> &Dr {
        &self.dr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x30 - A/D Channel n Data Register. This register contains the result of the most recent conversion completed on channel n."]
    #[inline(always)]
    pub fn dr_iter(&self) -> impl Iterator<Item = &Dr> {
        self.dr.iter()
    }
    #[doc = "0x30 - A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "CR (rw) register accessor: A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
pub mod cr;
#[doc = "GDR (rw) register accessor: A/D Global Data Register. Contains the result of the most recent A/D conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdr`]
module"]
#[doc(alias = "GDR")]
pub type Gdr = crate::Reg<gdr::GdrSpec>;
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion."]
pub mod gdr;
#[doc = "STAT (r) register accessor: A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
pub mod stat;
#[doc = "INTEN (rw) register accessor: A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub mod inten;
#[doc = "DR (rw) register accessor: A/D Channel n Data Register. This register contains the result of the most recent conversion completed on channel n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "A/D Channel n Data Register. This register contains the result of the most recent conversion completed on channel n."]
pub mod dr;
