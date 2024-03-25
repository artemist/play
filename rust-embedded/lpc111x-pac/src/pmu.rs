#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcon: Pcon,
    gpreg: [Gpreg; 4],
    gpreg4: Gpreg4,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register"]
    #[inline(always)]
    pub const fn pcon(&self) -> &Pcon {
        &self.pcon
    }
    #[doc = "0x04..0x14 - General purpose register"]
    #[inline(always)]
    pub const fn gpreg(&self, n: usize) -> &Gpreg {
        &self.gpreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - General purpose register"]
    #[inline(always)]
    pub fn gpreg_iter(&self) -> impl Iterator<Item = &Gpreg> {
        self.gpreg.iter()
    }
    #[doc = "0x14 - General purpose register 4"]
    #[inline(always)]
    pub const fn gpreg4(&self) -> &Gpreg4 {
        &self.gpreg4
    }
}
#[doc = "PCON (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcon`]
module"]
#[doc(alias = "PCON")]
pub type Pcon = crate::Reg<pcon::PconSpec>;
#[doc = "Power control register"]
pub mod pcon;
#[doc = "GPREG (rw) register accessor: General purpose register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg`]
module"]
#[doc(alias = "GPREG")]
pub type Gpreg = crate::Reg<gpreg::GpregSpec>;
#[doc = "General purpose register"]
pub mod gpreg;
#[doc = "GPREG4 (rw) register accessor: General purpose register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg4`]
module"]
#[doc(alias = "GPREG4")]
pub type Gpreg4 = crate::Reg<gpreg4::Gpreg4Spec>;
#[doc = "General purpose register 4"]
pub mod gpreg4;
