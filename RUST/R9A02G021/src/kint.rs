#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    krctl: Krctl,
    _reserved1: [u8; 0x03],
    krf: Krf,
    _reserved2: [u8; 0x03],
    krm: Krm,
}
impl RegisterBlock {
    #[doc = "0x00 - Key Return Control Register"]
    #[inline(always)]
    pub const fn krctl(&self) -> &Krctl {
        &self.krctl
    }
    #[doc = "0x04 - Key Return Flag Register"]
    #[inline(always)]
    pub const fn krf(&self) -> &Krf {
        &self.krf
    }
    #[doc = "0x08 - Key Return Mode Register"]
    #[inline(always)]
    pub const fn krm(&self) -> &Krm {
        &self.krm
    }
}
#[doc = "KRCTL (rw) register accessor: Key Return Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krctl`]
module"]
#[doc(alias = "KRCTL")]
pub type Krctl = crate::Reg<krctl::KrctlSpec>;
#[doc = "Key Return Control Register"]
pub mod krctl;
#[doc = "KRF (rw) register accessor: Key Return Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krf`]
module"]
#[doc(alias = "KRF")]
pub type Krf = crate::Reg<krf::KrfSpec>;
#[doc = "Key Return Flag Register"]
pub mod krf;
#[doc = "KRM (rw) register accessor: Key Return Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krm`]
module"]
#[doc(alias = "KRM")]
pub type Krm = crate::Reg<krm::KrmSpec>;
#[doc = "Key Return Mode Register"]
pub mod krm;
