#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    snfen: Snfen,
    tnfen: Tnfen,
    _reserved2: [u8; 0x01],
    isc: Isc,
    tis0: Tis0,
    _reserved4: [u8; 0x04],
    ulbs: Ulbs,
    _reserved5: [u8; 0x3e],
    ccde: Ccde,
    _reserved6: [u8; 0x38],
    cctrm: Cctrm,
}
impl RegisterBlock {
    #[doc = "0x00 - SAU Noise Filter Enable Register"]
    #[inline(always)]
    pub const fn snfen(&self) -> &Snfen {
        &self.snfen
    }
    #[doc = "0x01 - TAU Noise Filter Enable Register"]
    #[inline(always)]
    pub const fn tnfen(&self) -> &Tnfen {
        &self.tnfen
    }
    #[doc = "0x03 - Input Switch Control Register"]
    #[inline(always)]
    pub const fn isc(&self) -> &Isc {
        &self.isc
    }
    #[doc = "0x04 - Timer Input Select Register 0"]
    #[inline(always)]
    pub const fn tis0(&self) -> &Tis0 {
        &self.tis0
    }
    #[doc = "0x09 - UART Loopback Select Register"]
    #[inline(always)]
    pub const fn ulbs(&self) -> &Ulbs {
        &self.ulbs
    }
    #[doc = "0x48 - Output Current Control Enable Register"]
    #[inline(always)]
    pub const fn ccde(&self) -> &Ccde {
        &self.ccde
    }
    #[doc = "0x81 - Output Current Control Trimming Register"]
    #[inline(always)]
    pub const fn cctrm(&self) -> &Cctrm {
        &self.cctrm
    }
}
#[doc = "SNFEN (rw) register accessor: SAU Noise Filter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snfen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snfen`]
module"]
#[doc(alias = "SNFEN")]
pub type Snfen = crate::Reg<snfen::SnfenSpec>;
#[doc = "SAU Noise Filter Enable Register"]
pub mod snfen;
#[doc = "TNFEN (rw) register accessor: TAU Noise Filter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tnfen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tnfen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnfen`]
module"]
#[doc(alias = "TNFEN")]
pub type Tnfen = crate::Reg<tnfen::TnfenSpec>;
#[doc = "TAU Noise Filter Enable Register"]
pub mod tnfen;
#[doc = "ISC (rw) register accessor: Input Switch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc`]
module"]
#[doc(alias = "ISC")]
pub type Isc = crate::Reg<isc::IscSpec>;
#[doc = "Input Switch Control Register"]
pub mod isc;
#[doc = "TIS0 (rw) register accessor: Timer Input Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tis0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tis0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tis0`]
module"]
#[doc(alias = "TIS0")]
pub type Tis0 = crate::Reg<tis0::Tis0Spec>;
#[doc = "Timer Input Select Register 0"]
pub mod tis0;
#[doc = "ULBS (rw) register accessor: UART Loopback Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ulbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulbs`]
module"]
#[doc(alias = "ULBS")]
pub type Ulbs = crate::Reg<ulbs::UlbsSpec>;
#[doc = "UART Loopback Select Register"]
pub mod ulbs;
#[doc = "CCDE (rw) register accessor: Output Current Control Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccde::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccde::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccde`]
module"]
#[doc(alias = "CCDE")]
pub type Ccde = crate::Reg<ccde::CcdeSpec>;
#[doc = "Output Current Control Enable Register"]
pub mod ccde;
#[doc = "CCTRM (rw) register accessor: Output Current Control Trimming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrm`]
module"]
#[doc(alias = "CCTRM")]
pub type Cctrm = crate::Reg<cctrm::CctrmSpec>;
#[doc = "Output Current Control Trimming Register"]
pub mod cctrm;
