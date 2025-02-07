#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mactcr: Mactcr,
    _reserved1: [u8; 0xfc],
    swrcr: Swrcr,
    _reserved2: [u8; 0xfc],
    nmiaddr: Nmiaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Machine Timer Control Register"]
    #[inline(always)]
    pub const fn mactcr(&self) -> &Mactcr {
        &self.mactcr
    }
    #[doc = "0x100 - Software Reset Control Register"]
    #[inline(always)]
    pub const fn swrcr(&self) -> &Swrcr {
        &self.swrcr
    }
    #[doc = "0x200 - NMI Hander Address Register"]
    #[inline(always)]
    pub const fn nmiaddr(&self) -> &Nmiaddr {
        &self.nmiaddr
    }
}
#[doc = "MACTCR (rw) register accessor: Machine Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactcr`]
module"]
#[doc(alias = "MACTCR")]
pub type Mactcr = crate::Reg<mactcr::MactcrSpec>;
#[doc = "Machine Timer Control Register"]
pub mod mactcr;
#[doc = "SWRCR (rw) register accessor: Software Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrcr`]
module"]
#[doc(alias = "SWRCR")]
pub type Swrcr = crate::Reg<swrcr::SwrcrSpec>;
#[doc = "Software Reset Control Register"]
pub mod swrcr;
#[doc = "NMIADDR (rw) register accessor: NMI Hander Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiaddr`]
module"]
#[doc(alias = "NMIADDR")]
pub type Nmiaddr = crate::Reg<nmiaddr::NmiaddrSpec>;
#[doc = "NMI Hander Address Register"]
pub mod nmiaddr;
