#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    irqcr: [Irqcr; 8],
    _reserved1: [u8; 0xf8],
    nmicr: Nmicr,
    _reserved2: [u8; 0x1f],
    nmier: Nmier,
    _reserved3: [u8; 0x0e],
    nmiclr: Nmiclr,
    _reserved4: [u8; 0x0e],
    nmisr: Nmisr,
    _reserved5: [u8; 0x5e],
    wupen0: Wupen0,
    wupen1: Wupen1,
    _reserved7: [u8; 0x18],
    ielen: Ielen,
    _reserved8: [u8; 0x3f],
    selsr0: Selsr0,
    _reserved9: [u8; 0xfe],
    ielsr: [Ielsr; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - IRQ Control Register %s"]
    #[inline(always)]
    pub const fn irqcr(&self, n: usize) -> &Irqcr {
        &self.irqcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - IRQ Control Register %s"]
    #[inline(always)]
    pub fn irqcr_iter(&self) -> impl Iterator<Item = &Irqcr> {
        self.irqcr.iter()
    }
    #[doc = "0x100 - NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &Nmicr {
        &self.nmicr
    }
    #[doc = "0x120 - Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &Nmier {
        &self.nmier
    }
    #[doc = "0x130 - Non-Maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(&self) -> &Nmiclr {
        &self.nmiclr
    }
    #[doc = "0x140 - Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &Nmisr {
        &self.nmisr
    }
    #[doc = "0x1a0 - Wake Up Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn wupen0(&self) -> &Wupen0 {
        &self.wupen0
    }
    #[doc = "0x1a4 - Wake Up Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn wupen1(&self) -> &Wupen1 {
        &self.wupen1
    }
    #[doc = "0x1c0 - ICU Event Enable Register"]
    #[inline(always)]
    pub const fn ielen(&self) -> &Ielen {
        &self.ielen
    }
    #[doc = "0x200 - SYS Event Link Setting Register"]
    #[inline(always)]
    pub const fn selsr0(&self) -> &Selsr0 {
        &self.selsr0
    }
    #[doc = "0x300..0x380 - ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(&self, n: usize) -> &Ielsr {
        &self.ielsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x380 - ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub fn ielsr_iter(&self) -> impl Iterator<Item = &Ielsr> {
        self.ielsr.iter()
    }
}
#[doc = "IRQCR (rw) register accessor: IRQ Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`irqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqcr`]
module"]
#[doc(alias = "IRQCR")]
pub type Irqcr = crate::Reg<irqcr::IrqcrSpec>;
#[doc = "IRQ Control Register %s"]
pub mod irqcr;
#[doc = "NMICR (rw) register accessor: NMI Pin Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicr`]
module"]
#[doc(alias = "NMICR")]
pub type Nmicr = crate::Reg<nmicr::NmicrSpec>;
#[doc = "NMI Pin Interrupt Control Register"]
pub mod nmicr;
#[doc = "NMIER (rw) register accessor: Non-Maskable Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmier`]
module"]
#[doc(alias = "NMIER")]
pub type Nmier = crate::Reg<nmier::NmierSpec>;
#[doc = "Non-Maskable Interrupt Enable Register"]
pub mod nmier;
#[doc = "NMICLR (rw) register accessor: Non-Maskable Interrupt Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiclr`]
module"]
#[doc(alias = "NMICLR")]
pub type Nmiclr = crate::Reg<nmiclr::NmiclrSpec>;
#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub mod nmiclr;
#[doc = "NMISR (r) register accessor: Non-Maskable Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisr`]
module"]
#[doc(alias = "NMISR")]
pub type Nmisr = crate::Reg<nmisr::NmisrSpec>;
#[doc = "Non-Maskable Interrupt Status Register"]
pub mod nmisr;
#[doc = "WUPEN0 (rw) register accessor: Wake Up Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupen0`]
module"]
#[doc(alias = "WUPEN0")]
pub type Wupen0 = crate::Reg<wupen0::Wupen0Spec>;
#[doc = "Wake Up Interrupt Enable Register 0"]
pub mod wupen0;
#[doc = "WUPEN1 (rw) register accessor: Wake Up Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupen1`]
module"]
#[doc(alias = "WUPEN1")]
pub type Wupen1 = crate::Reg<wupen1::Wupen1Spec>;
#[doc = "Wake Up Interrupt Enable Register 1"]
pub mod wupen1;
#[doc = "IELEN (rw) register accessor: ICU Event Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ielen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ielen`]
module"]
#[doc(alias = "IELEN")]
pub type Ielen = crate::Reg<ielen::IelenSpec>;
#[doc = "ICU Event Enable Register"]
pub mod ielen;
#[doc = "SELSR0 (rw) register accessor: SYS Event Link Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selsr0`]
module"]
#[doc(alias = "SELSR0")]
pub type Selsr0 = crate::Reg<selsr0::Selsr0Spec>;
#[doc = "SYS Event Link Setting Register"]
pub mod selsr0;
#[doc = "IELSR (rw) register accessor: ICU Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ielsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ielsr`]
module"]
#[doc(alias = "IELSR")]
pub type Ielsr = crate::Reg<ielsr::IelsrSpec>;
#[doc = "ICU Event Link Setting Register %s"]
pub mod ielsr;
