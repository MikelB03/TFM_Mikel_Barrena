#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mtime_lo: MtimeLo,
    mtime_hi: MtimeHi,
    mtimecmp_lo: MtimecmpLo,
    mtimecmp_hi: MtimecmpHi,
    _reserved4: [u8; 0x0fec],
    msip: Msip,
}
impl RegisterBlock {
    #[doc = "0x00 - Machine Timer Counter Register Low"]
    #[inline(always)]
    pub const fn mtime_lo(&self) -> &MtimeLo {
        &self.mtime_lo
    }
    #[doc = "0x04 - Machine Timer Counter Register High"]
    #[inline(always)]
    pub const fn mtime_hi(&self) -> &MtimeHi {
        &self.mtime_hi
    }
    #[doc = "0x08 - Machine Timer Comparator Register 0 Low"]
    #[inline(always)]
    pub const fn mtimecmp_lo(&self) -> &MtimecmpLo {
        &self.mtimecmp_lo
    }
    #[doc = "0x0c - Machine Timer Comparator Register 0 High"]
    #[inline(always)]
    pub const fn mtimecmp_hi(&self) -> &MtimecmpHi {
        &self.mtimecmp_hi
    }
    #[doc = "0xffc - Triggering Software Interrupt"]
    #[inline(always)]
    pub const fn msip(&self) -> &Msip {
        &self.msip
    }
}
#[doc = "mtime_lo (rw) register accessor: Machine Timer Counter Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_lo`]
module"]
#[doc(alias = "mtime_lo")]
pub type MtimeLo = crate::Reg<mtime_lo::MtimeLoSpec>;
#[doc = "Machine Timer Counter Register Low"]
pub mod mtime_lo;
#[doc = "mtime_hi (rw) register accessor: Machine Timer Counter Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_hi`]
module"]
#[doc(alias = "mtime_hi")]
pub type MtimeHi = crate::Reg<mtime_hi::MtimeHiSpec>;
#[doc = "Machine Timer Counter Register High"]
pub mod mtime_hi;
#[doc = "mtimecmp_lo (rw) register accessor: Machine Timer Comparator Register 0 Low\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_lo`]
module"]
#[doc(alias = "mtimecmp_lo")]
pub type MtimecmpLo = crate::Reg<mtimecmp_lo::MtimecmpLoSpec>;
#[doc = "Machine Timer Comparator Register 0 Low"]
pub mod mtimecmp_lo;
#[doc = "mtimecmp_hi (rw) register accessor: Machine Timer Comparator Register 0 High\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_hi`]
module"]
#[doc(alias = "mtimecmp_hi")]
pub type MtimecmpHi = crate::Reg<mtimecmp_hi::MtimecmpHiSpec>;
#[doc = "Machine Timer Comparator Register 0 High"]
pub mod mtimecmp_hi;
#[doc = "msip (rw) register accessor: Triggering Software Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "msip")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "Triggering Software Interrupt"]
pub mod msip;
