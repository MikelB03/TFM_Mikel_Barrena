#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    docr: Docr,
    _reserved1: [u8; 0x03],
    dosr: Dosr,
    _reserved2: [u8; 0x03],
    doscr: Doscr,
    _reserved3: [u8; 0x03],
    dodir: Dodir,
    dodsr0: Dodsr0,
    dodsr1: Dodsr1,
}
impl RegisterBlock {
    #[doc = "0x00 - DOC Control Register"]
    #[inline(always)]
    pub const fn docr(&self) -> &Docr {
        &self.docr
    }
    #[doc = "0x04 - DOC Flag Status Register"]
    #[inline(always)]
    pub const fn dosr(&self) -> &Dosr {
        &self.dosr
    }
    #[doc = "0x08 - DOC Flag Status Clear Register"]
    #[inline(always)]
    pub const fn doscr(&self) -> &Doscr {
        &self.doscr
    }
    #[doc = "0x0c - DOC Data Input Register"]
    #[inline(always)]
    pub const fn dodir(&self) -> &Dodir {
        &self.dodir
    }
    #[doc = "0x10 - DOC Data Setting Register 0"]
    #[inline(always)]
    pub const fn dodsr0(&self) -> &Dodsr0 {
        &self.dodsr0
    }
    #[doc = "0x14 - DOC Data Setting Register 1"]
    #[inline(always)]
    pub const fn dodsr1(&self) -> &Dodsr1 {
        &self.dodsr1
    }
}
#[doc = "DOCR (rw) register accessor: DOC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`docr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@docr`]
module"]
#[doc(alias = "DOCR")]
pub type Docr = crate::Reg<docr::DocrSpec>;
#[doc = "DOC Control Register"]
pub mod docr;
#[doc = "DOSR (r) register accessor: DOC Flag Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dosr`]
module"]
#[doc(alias = "DOSR")]
pub type Dosr = crate::Reg<dosr::DosrSpec>;
#[doc = "DOC Flag Status Register"]
pub mod dosr;
#[doc = "DOSCR (w) register accessor: DOC Flag Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doscr`]
module"]
#[doc(alias = "DOSCR")]
pub type Doscr = crate::Reg<doscr::DoscrSpec>;
#[doc = "DOC Flag Status Clear Register"]
pub mod doscr;
#[doc = "DODIR (rw) register accessor: DOC Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dodir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodir`]
module"]
#[doc(alias = "DODIR")]
pub type Dodir = crate::Reg<dodir::DodirSpec>;
#[doc = "DOC Data Input Register"]
pub mod dodir;
#[doc = "DODSR0 (rw) register accessor: DOC Data Setting Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dodsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodsr0`]
module"]
#[doc(alias = "DODSR0")]
pub type Dodsr0 = crate::Reg<dodsr0::Dodsr0Spec>;
#[doc = "DOC Data Setting Register 0"]
pub mod dodsr0;
#[doc = "DODSR1 (rw) register accessor: DOC Data Setting Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dodsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodsr1`]
module"]
#[doc(alias = "DODSR1")]
pub type Dodsr1 = crate::Reg<dodsr1::Dodsr1Spec>;
#[doc = "DOC Data Setting Register 1"]
pub mod dodsr1;
