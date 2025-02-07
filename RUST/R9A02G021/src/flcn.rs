#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x90],
    dflctl: Dflctl,
    _reserved1: [u8; 0x6f],
    fpmcr: Fpmcr,
    _reserved2: [u8; 0x03],
    fasr: Fasr,
    _reserved3: [u8; 0x03],
    fsarl: Fsarl,
    _reserved4: [u8; 0x06],
    fsarh: Fsarh,
    _reserved5: [u8; 0x02],
    fcr: Fcr,
    _reserved6: [u8; 0x03],
    fearl: Fearl,
    _reserved7: [u8; 0x06],
    fearh: Fearh,
    _reserved8: [u8; 0x02],
    fresetr: Fresetr,
    _reserved9: [u8; 0x03],
    fstatr00: Fstatr00,
    _reserved10: [u8; 0x02],
    fstatr1: Fstatr1,
    _reserved11: [u8; 0x03],
    fwbl0: Fwbl0,
    _reserved12: [u8; 0x06],
    fwbh0: Fwbh0,
    _reserved13: [u8; 0x06],
    fwbl1: Fwbl1,
    _reserved14: [u8; 0x02],
    fwbh1: Fwbh1,
    _reserved15: [u8; 0x3a],
    fpr: Fpr,
    _reserved16: [u8; 0x03],
    fpsr: Fpsr,
    _reserved17: [u8; 0x3b],
    fsecmr: Fsecmr,
    _reserved18: [u8; 0x06],
    fawsmr: Fawsmr,
    _reserved19: [u8; 0x06],
    fawemr: Fawemr,
    _reserved20: [u8; 0x06],
    fisr: Fisr,
    _reserved21: [u8; 0x03],
    fexcr: Fexcr,
    _reserved22: [u8; 0x03],
    feaml: Feaml,
    _reserved23: [u8; 0x06],
    feamh: Feamh,
    _reserved24: [u8; 0x3e],
    tscdr: Tscdr,
    _reserved25: [u8; 0x3d84],
    fentryr: Fentryr,
    _reserved26: [u8; 0x12],
    fldwaitr: Fldwaitr,
    _reserved27: [u8; 0x03],
    pfber: Pfber,
}
impl RegisterBlock {
    #[doc = "0x90 - Data Flash Control Register"]
    #[inline(always)]
    pub const fn dflctl(&self) -> &Dflctl {
        &self.dflctl
    }
    #[doc = "0x100 - Flash P/E Mode Control Register"]
    #[inline(always)]
    pub const fn fpmcr(&self) -> &Fpmcr {
        &self.fpmcr
    }
    #[doc = "0x104 - Flash Area Select Register"]
    #[inline(always)]
    pub const fn fasr(&self) -> &Fasr {
        &self.fasr
    }
    #[doc = "0x108 - Flash Processing Start Address Register L"]
    #[inline(always)]
    pub const fn fsarl(&self) -> &Fsarl {
        &self.fsarl
    }
    #[doc = "0x110 - Flash Processing Start Address Register H"]
    #[inline(always)]
    pub const fn fsarh(&self) -> &Fsarh {
        &self.fsarh
    }
    #[doc = "0x114 - Flash Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x118 - Flash Processing End Address Register L"]
    #[inline(always)]
    pub const fn fearl(&self) -> &Fearl {
        &self.fearl
    }
    #[doc = "0x120 - Flash Processing End Address Register H"]
    #[inline(always)]
    pub const fn fearh(&self) -> &Fearh {
        &self.fearh
    }
    #[doc = "0x124 - Flash Reset Register"]
    #[inline(always)]
    pub const fn fresetr(&self) -> &Fresetr {
        &self.fresetr
    }
    #[doc = "0x128 - Flash Status Register 0"]
    #[inline(always)]
    pub const fn fstatr00(&self) -> &Fstatr00 {
        &self.fstatr00
    }
    #[doc = "0x12c - Flash Status Register 1"]
    #[inline(always)]
    pub const fn fstatr1(&self) -> &Fstatr1 {
        &self.fstatr1
    }
    #[doc = "0x130 - Flash Write Buffer Register L0"]
    #[inline(always)]
    pub const fn fwbl0(&self) -> &Fwbl0 {
        &self.fwbl0
    }
    #[doc = "0x138 - Flash Write Buffer Register H0"]
    #[inline(always)]
    pub const fn fwbh0(&self) -> &Fwbh0 {
        &self.fwbh0
    }
    #[doc = "0x140 - Flash Write Buffer Register L1"]
    #[inline(always)]
    pub const fn fwbl1(&self) -> &Fwbl1 {
        &self.fwbl1
    }
    #[doc = "0x144 - Flash Write Buffer Register H1"]
    #[inline(always)]
    pub const fn fwbh1(&self) -> &Fwbh1 {
        &self.fwbh1
    }
    #[doc = "0x180 - Protection Unlock Register"]
    #[inline(always)]
    pub const fn fpr(&self) -> &Fpr {
        &self.fpr
    }
    #[doc = "0x184 - Protection Unlock Status Register"]
    #[inline(always)]
    pub const fn fpsr(&self) -> &Fpsr {
        &self.fpsr
    }
    #[doc = "0x1c0 - Flash Protection Flag Monitor Register"]
    #[inline(always)]
    pub const fn fsecmr(&self) -> &Fsecmr {
        &self.fsecmr
    }
    #[doc = "0x1c8 - Flash Access Window Start Address Monitor Register"]
    #[inline(always)]
    pub const fn fawsmr(&self) -> &Fawsmr {
        &self.fawsmr
    }
    #[doc = "0x1d0 - Flash Access Window End Address Monitor Register"]
    #[inline(always)]
    pub const fn fawemr(&self) -> &Fawemr {
        &self.fawemr
    }
    #[doc = "0x1d8 - Flash Initial Setting Register"]
    #[inline(always)]
    pub const fn fisr(&self) -> &Fisr {
        &self.fisr
    }
    #[doc = "0x1dc - Flash Extra Area Control Register"]
    #[inline(always)]
    pub const fn fexcr(&self) -> &Fexcr {
        &self.fexcr
    }
    #[doc = "0x1e0 - Flash Error Address Monitor Register L"]
    #[inline(always)]
    pub const fn feaml(&self) -> &Feaml {
        &self.feaml
    }
    #[doc = "0x1e8 - Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub const fn feamh(&self) -> &Feamh {
        &self.feamh
    }
    #[doc = "0x228 - Temperature Sensor Calibration Data Register"]
    #[inline(always)]
    pub const fn tscdr(&self) -> &Tscdr {
        &self.tscdr
    }
    #[doc = "0x3fb0 - Flash P/E Mode Entry Register"]
    #[inline(always)]
    pub const fn fentryr(&self) -> &Fentryr {
        &self.fentryr
    }
    #[doc = "0x3fc4 - Memory Wait Cycle Control Register for Data Flash"]
    #[inline(always)]
    pub const fn fldwaitr(&self) -> &Fldwaitr {
        &self.fldwaitr
    }
    #[doc = "0x3fc8 - Prefetch Buffer Enable Register"]
    #[inline(always)]
    pub const fn pfber(&self) -> &Pfber {
        &self.pfber
    }
}
#[doc = "DFLCTL (rw) register accessor: Data Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dflctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dflctl`]
module"]
#[doc(alias = "DFLCTL")]
pub type Dflctl = crate::Reg<dflctl::DflctlSpec>;
#[doc = "Data Flash Control Register"]
pub mod dflctl;
#[doc = "FPMCR (rw) register accessor: Flash P/E Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpmcr`]
module"]
#[doc(alias = "FPMCR")]
pub type Fpmcr = crate::Reg<fpmcr::FpmcrSpec>;
#[doc = "Flash P/E Mode Control Register"]
pub mod fpmcr;
#[doc = "FASR (rw) register accessor: Flash Area Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fasr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fasr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fasr`]
module"]
#[doc(alias = "FASR")]
pub type Fasr = crate::Reg<fasr::FasrSpec>;
#[doc = "Flash Area Select Register"]
pub mod fasr;
#[doc = "FSARL (rw) register accessor: Flash Processing Start Address Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`fsarl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsarl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsarl`]
module"]
#[doc(alias = "FSARL")]
pub type Fsarl = crate::Reg<fsarl::FsarlSpec>;
#[doc = "Flash Processing Start Address Register L"]
pub mod fsarl;
#[doc = "FSARH (rw) register accessor: Flash Processing Start Address Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`fsarh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsarh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsarh`]
module"]
#[doc(alias = "FSARH")]
pub type Fsarh = crate::Reg<fsarh::FsarhSpec>;
#[doc = "Flash Processing Start Address Register H"]
pub mod fsarh;
#[doc = "FCR (rw) register accessor: Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "Flash Control Register"]
pub mod fcr;
#[doc = "FEARL (rw) register accessor: Flash Processing End Address Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`fearl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fearl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fearl`]
module"]
#[doc(alias = "FEARL")]
pub type Fearl = crate::Reg<fearl::FearlSpec>;
#[doc = "Flash Processing End Address Register L"]
pub mod fearl;
#[doc = "FEARH (rw) register accessor: Flash Processing End Address Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`fearh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fearh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fearh`]
module"]
#[doc(alias = "FEARH")]
pub type Fearh = crate::Reg<fearh::FearhSpec>;
#[doc = "Flash Processing End Address Register H"]
pub mod fearh;
#[doc = "FRESETR (rw) register accessor: Flash Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fresetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fresetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fresetr`]
module"]
#[doc(alias = "FRESETR")]
pub type Fresetr = crate::Reg<fresetr::FresetrSpec>;
#[doc = "Flash Reset Register"]
pub mod fresetr;
#[doc = "FSTATR00 (r) register accessor: Flash Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fstatr00::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstatr00`]
module"]
#[doc(alias = "FSTATR00")]
pub type Fstatr00 = crate::Reg<fstatr00::Fstatr00Spec>;
#[doc = "Flash Status Register 0"]
pub mod fstatr00;
#[doc = "FSTATR1 (r) register accessor: Flash Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fstatr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstatr1`]
module"]
#[doc(alias = "FSTATR1")]
pub type Fstatr1 = crate::Reg<fstatr1::Fstatr1Spec>;
#[doc = "Flash Status Register 1"]
pub mod fstatr1;
#[doc = "FWBL0 (rw) register accessor: Flash Write Buffer Register L0\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbl0`]
module"]
#[doc(alias = "FWBL0")]
pub type Fwbl0 = crate::Reg<fwbl0::Fwbl0Spec>;
#[doc = "Flash Write Buffer Register L0"]
pub mod fwbl0;
#[doc = "FWBH0 (rw) register accessor: Flash Write Buffer Register H0\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbh0`]
module"]
#[doc(alias = "FWBH0")]
pub type Fwbh0 = crate::Reg<fwbh0::Fwbh0Spec>;
#[doc = "Flash Write Buffer Register H0"]
pub mod fwbh0;
#[doc = "FWBL1 (rw) register accessor: Flash Write Buffer Register L1\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbl1`]
module"]
#[doc(alias = "FWBL1")]
pub type Fwbl1 = crate::Reg<fwbl1::Fwbl1Spec>;
#[doc = "Flash Write Buffer Register L1"]
pub mod fwbl1;
#[doc = "FWBH1 (rw) register accessor: Flash Write Buffer Register H1\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbh1`]
module"]
#[doc(alias = "FWBH1")]
pub type Fwbh1 = crate::Reg<fwbh1::Fwbh1Spec>;
#[doc = "Flash Write Buffer Register H1"]
pub mod fwbh1;
#[doc = "FPR (rw) register accessor: Protection Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr`]
module"]
#[doc(alias = "FPR")]
pub type Fpr = crate::Reg<fpr::FprSpec>;
#[doc = "Protection Unlock Register"]
pub mod fpr;
#[doc = "FPSR (r) register accessor: Protection Unlock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpsr`]
module"]
#[doc(alias = "FPSR")]
pub type Fpsr = crate::Reg<fpsr::FpsrSpec>;
#[doc = "Protection Unlock Status Register"]
pub mod fpsr;
#[doc = "FSECMR (r) register accessor: Flash Protection Flag Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsecmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsecmr`]
module"]
#[doc(alias = "FSECMR")]
pub type Fsecmr = crate::Reg<fsecmr::FsecmrSpec>;
#[doc = "Flash Protection Flag Monitor Register"]
pub mod fsecmr;
#[doc = "FAWSMR (r) register accessor: Flash Access Window Start Address Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fawsmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fawsmr`]
module"]
#[doc(alias = "FAWSMR")]
pub type Fawsmr = crate::Reg<fawsmr::FawsmrSpec>;
#[doc = "Flash Access Window Start Address Monitor Register"]
pub mod fawsmr;
#[doc = "FAWEMR (r) register accessor: Flash Access Window End Address Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fawemr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fawemr`]
module"]
#[doc(alias = "FAWEMR")]
pub type Fawemr = crate::Reg<fawemr::FawemrSpec>;
#[doc = "Flash Access Window End Address Monitor Register"]
pub mod fawemr;
#[doc = "FISR (rw) register accessor: Flash Initial Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fisr`]
module"]
#[doc(alias = "FISR")]
pub type Fisr = crate::Reg<fisr::FisrSpec>;
#[doc = "Flash Initial Setting Register"]
pub mod fisr;
#[doc = "FEXCR (rw) register accessor: Flash Extra Area Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fexcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fexcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fexcr`]
module"]
#[doc(alias = "FEXCR")]
pub type Fexcr = crate::Reg<fexcr::FexcrSpec>;
#[doc = "Flash Extra Area Control Register"]
pub mod fexcr;
#[doc = "FEAML (rw) register accessor: Flash Error Address Monitor Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`feaml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feaml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feaml`]
module"]
#[doc(alias = "FEAML")]
pub type Feaml = crate::Reg<feaml::FeamlSpec>;
#[doc = "Flash Error Address Monitor Register L"]
pub mod feaml;
#[doc = "FEAMH (rw) register accessor: Flash Error Address Monitor Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`feamh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feamh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feamh`]
module"]
#[doc(alias = "FEAMH")]
pub type Feamh = crate::Reg<feamh::FeamhSpec>;
#[doc = "Flash Error Address Monitor Register H"]
pub mod feamh;
#[doc = "TSCDR (r) register accessor: Temperature Sensor Calibration Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscdr`]
module"]
#[doc(alias = "TSCDR")]
pub type Tscdr = crate::Reg<tscdr::TscdrSpec>;
#[doc = "Temperature Sensor Calibration Data Register"]
pub mod tscdr;
#[doc = "FENTRYR (rw) register accessor: Flash P/E Mode Entry Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fentryr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fentryr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fentryr`]
module"]
#[doc(alias = "FENTRYR")]
pub type Fentryr = crate::Reg<fentryr::FentryrSpec>;
#[doc = "Flash P/E Mode Entry Register"]
pub mod fentryr;
#[doc = "FLDWAITR (rw) register accessor: Memory Wait Cycle Control Register for Data Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`fldwaitr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fldwaitr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fldwaitr`]
module"]
#[doc(alias = "FLDWAITR")]
pub type Fldwaitr = crate::Reg<fldwaitr::FldwaitrSpec>;
#[doc = "Memory Wait Cycle Control Register for Data Flash"]
pub mod fldwaitr;
#[doc = "PFBER (rw) register accessor: Prefetch Buffer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfber`]
module"]
#[doc(alias = "PFBER")]
pub type Pfber = crate::Reg<pfber::PfberSpec>;
#[doc = "Prefetch Buffer Enable Register"]
pub mod pfber;
