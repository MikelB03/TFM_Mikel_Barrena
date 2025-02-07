#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sbycr: Sbycr,
    _reserved1: [u8; 0x0e],
    mstpcra: Mstpcra,
    sckdivcr: Sckdivcr,
    _reserved3: [u8; 0x02],
    sckscr: Sckscr,
    _reserved4: [u8; 0x0a],
    memwait: Memwait,
    mosccr: Mosccr,
    _reserved6: [u8; 0x03],
    hococr: Hococr,
    hococr2: Hococr2,
    mococr: Mococr,
    _reserved9: [u8; 0x03],
    oscsf: Oscsf,
    _reserved10: [u8; 0x01],
    ckocr: Ckocr,
    _reserved11: [u8; 0x0d],
    lpopt: Lpopt,
    _reserved12: [u8; 0x07],
    osmcr: Osmcr,
    _reserved13: [u8; 0x0c],
    mocoutcr: Mocoutcr,
    hocoutcr: Hocoutcr,
    _reserved15: [u8; 0x2f],
    snzcr: Snzcr,
    _reserved16: [u8; 0x01],
    snzedcr0: Snzedcr0,
    snzedcr1: Snzedcr1,
    _reserved18: [u8; 0x02],
    snzreqcr0: Snzreqcr0,
    _reserved19: [u8; 0x03],
    psmcr: Psmcr,
    opccr: Opccr,
    _reserved21: [u8; 0x09],
    sopccr: Sopccr,
    _reserved22: [u8; 0x15],
    rstsr1: Rstsr1,
    _reserved23: [u8; 0x1e],
    lvd1cr1: Lvd1cr1,
    lvd1sr: Lvd1sr,
    lvd2cr1: Lvd2cr1,
    lvd2sr: Lvd2sr,
    _reserved27: [u8; 0x031a],
    prcr: Prcr,
    _reserved28: [u8; 0x0e],
    syocdcr: Syocdcr,
    _reserved29: [u8; 0x01],
    rstsr0: Rstsr0,
    rstsr2: Rstsr2,
    _reserved31: [u8; 0x05],
    lvcmpcr: Lvcmpcr,
    lvdlvlr: Lvdlvlr,
    _reserved33: [u8; 0x01],
    lvd1cr0: Lvd1cr0,
    lvd2cr0: Lvd2cr0,
    _reserved35: [u8; 0x64],
    sosccr: Sosccr,
    somcr: Somcr,
    somrg: Somrg,
    _reserved38: [u8; 0x0d],
    lococr: Lococr,
    _reserved39: [u8; 0x01],
    locoutcr: Locoutcr,
}
impl RegisterBlock {
    #[doc = "0x0c - Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &Sbycr {
        &self.sbycr
    }
    #[doc = "0x1c - Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(&self) -> &Mstpcra {
        &self.mstpcra
    }
    #[doc = "0x20 - System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(&self) -> &Sckdivcr {
        &self.sckdivcr
    }
    #[doc = "0x26 - System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(&self) -> &Sckscr {
        &self.sckscr
    }
    #[doc = "0x31 - Memory Wait Cycle Control Register for Code Flash"]
    #[inline(always)]
    pub const fn memwait(&self) -> &Memwait {
        &self.memwait
    }
    #[doc = "0x32 - External Clock Input Control Register"]
    #[inline(always)]
    pub const fn mosccr(&self) -> &Mosccr {
        &self.mosccr
    }
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(&self) -> &Hococr {
        &self.hococr
    }
    #[doc = "0x37 - High-Speed On-Chip Oscillator Control Register 2"]
    #[inline(always)]
    pub const fn hococr2(&self) -> &Hococr2 {
        &self.hococr2
    }
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(&self) -> &Mococr {
        &self.mococr
    }
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &Oscsf {
        &self.oscsf
    }
    #[doc = "0x3e - Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(&self) -> &Ckocr {
        &self.ckocr
    }
    #[doc = "0x4c - Lower Power Operation Control Register"]
    #[inline(always)]
    pub const fn lpopt(&self) -> &Lpopt {
        &self.lpopt
    }
    #[doc = "0x54 - Subsystem Clock Supply Mode Control Register"]
    #[inline(always)]
    pub const fn osmcr(&self) -> &Osmcr {
        &self.osmcr
    }
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(&self) -> &Mocoutcr {
        &self.mocoutcr
    }
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(&self) -> &Hocoutcr {
        &self.hocoutcr
    }
    #[doc = "0x92 - Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(&self) -> &Snzcr {
        &self.snzcr
    }
    #[doc = "0x94 - Snooze End Control Register 0"]
    #[inline(always)]
    pub const fn snzedcr0(&self) -> &Snzedcr0 {
        &self.snzedcr0
    }
    #[doc = "0x95 - Snooze End Control Register 1"]
    #[inline(always)]
    pub const fn snzedcr1(&self) -> &Snzedcr1 {
        &self.snzedcr1
    }
    #[doc = "0x98 - Snooze Request Control Register 0"]
    #[inline(always)]
    pub const fn snzreqcr0(&self) -> &Snzreqcr0 {
        &self.snzreqcr0
    }
    #[doc = "0x9f - Power Save Memory Control Register"]
    #[inline(always)]
    pub const fn psmcr(&self) -> &Psmcr {
        &self.psmcr
    }
    #[doc = "0xa0 - Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(&self) -> &Opccr {
        &self.opccr
    }
    #[doc = "0xaa - Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(&self) -> &Sopccr {
        &self.sopccr
    }
    #[doc = "0xc0 - Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(&self) -> &Rstsr1 {
        &self.rstsr1
    }
    #[doc = "0xe0 - Voltage Monitor 1 Circuit Control Register"]
    #[inline(always)]
    pub const fn lvd1cr1(&self) -> &Lvd1cr1 {
        &self.lvd1cr1
    }
    #[doc = "0xe1 - Voltage Monitor 1 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd1sr(&self) -> &Lvd1sr {
        &self.lvd1sr
    }
    #[doc = "0xe2 - Voltage Monitor 2 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd2cr1(&self) -> &Lvd2cr1 {
        &self.lvd2cr1
    }
    #[doc = "0xe3 - Voltage Monitor 2 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd2sr(&self) -> &Lvd2sr {
        &self.lvd2sr
    }
    #[doc = "0x3fe - Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &Prcr {
        &self.prcr
    }
    #[doc = "0x40e - System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(&self) -> &Syocdcr {
        &self.syocdcr
    }
    #[doc = "0x410 - Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(&self) -> &Rstsr0 {
        &self.rstsr0
    }
    #[doc = "0x411 - Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(&self) -> &Rstsr2 {
        &self.rstsr2
    }
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(&self) -> &Lvcmpcr {
        &self.lvcmpcr
    }
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(&self) -> &Lvdlvlr {
        &self.lvdlvlr
    }
    #[doc = "0x41a - Voltage Monitor 1 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd1cr0(&self) -> &Lvd1cr0 {
        &self.lvd1cr0
    }
    #[doc = "0x41b - Voltage Monitor 2 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd2cr0(&self) -> &Lvd2cr0 {
        &self.lvd2cr0
    }
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(&self) -> &Sosccr {
        &self.sosccr
    }
    #[doc = "0x481 - Sub-Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &Somcr {
        &self.somcr
    }
    #[doc = "0x482 - Sub-Clock Oscillator Margin Check Register"]
    #[inline(always)]
    pub const fn somrg(&self) -> &Somrg {
        &self.somrg
    }
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(&self) -> &Lococr {
        &self.lococr
    }
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(&self) -> &Locoutcr {
        &self.locoutcr
    }
}
#[doc = "SBYCR (rw) register accessor: Standby Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbycr`]
module"]
#[doc(alias = "SBYCR")]
pub type Sbycr = crate::Reg<sbycr::SbycrSpec>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "MSTPCRA (rw) register accessor: Module Stop Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcra`]
module"]
#[doc(alias = "MSTPCRA")]
pub type Mstpcra = crate::Reg<mstpcra::MstpcraSpec>;
#[doc = "Module Stop Control Register A"]
pub mod mstpcra;
#[doc = "SCKDIVCR (rw) register accessor: System Clock Division Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckdivcr`]
module"]
#[doc(alias = "SCKDIVCR")]
pub type Sckdivcr = crate::Reg<sckdivcr::SckdivcrSpec>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKSCR (rw) register accessor: System Clock Source Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckscr`]
module"]
#[doc(alias = "SCKSCR")]
pub type Sckscr = crate::Reg<sckscr::SckscrSpec>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "MEMWAIT (rw) register accessor: Memory Wait Cycle Control Register for Code Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`memwait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memwait`]
module"]
#[doc(alias = "MEMWAIT")]
pub type Memwait = crate::Reg<memwait::MemwaitSpec>;
#[doc = "Memory Wait Cycle Control Register for Code Flash"]
pub mod memwait;
#[doc = "MOSCCR (rw) register accessor: External Clock Input Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosccr`]
module"]
#[doc(alias = "MOSCCR")]
pub type Mosccr = crate::Reg<mosccr::MosccrSpec>;
#[doc = "External Clock Input Control Register"]
pub mod mosccr;
#[doc = "HOCOCR (rw) register accessor: High-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hococr`]
module"]
#[doc(alias = "HOCOCR")]
pub type Hococr = crate::Reg<hococr::HococrSpec>;
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub mod hococr;
#[doc = "HOCOCR2 (rw) register accessor: High-Speed On-Chip Oscillator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hococr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hococr2`]
module"]
#[doc(alias = "HOCOCR2")]
pub type Hococr2 = crate::Reg<hococr2::Hococr2Spec>;
#[doc = "High-Speed On-Chip Oscillator Control Register 2"]
pub mod hococr2;
#[doc = "MOCOCR (rw) register accessor: Middle-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mococr`]
module"]
#[doc(alias = "MOCOCR")]
pub type Mococr = crate::Reg<mococr::MococrSpec>;
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub mod mococr;
#[doc = "OSCSF (r) register accessor: Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsf`]
module"]
#[doc(alias = "OSCSF")]
pub type Oscsf = crate::Reg<oscsf::OscsfSpec>;
#[doc = "Oscillation Stabilization Flag Register"]
pub mod oscsf;
#[doc = "CKOCR (rw) register accessor: Clock Out Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckocr`]
module"]
#[doc(alias = "CKOCR")]
pub type Ckocr = crate::Reg<ckocr::CkocrSpec>;
#[doc = "Clock Out Control Register"]
pub mod ckocr;
#[doc = "LPOPT (rw) register accessor: Lower Power Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpopt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpopt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpopt`]
module"]
#[doc(alias = "LPOPT")]
pub type Lpopt = crate::Reg<lpopt::LpoptSpec>;
#[doc = "Lower Power Operation Control Register"]
pub mod lpopt;
#[doc = "OSMCR (rw) register accessor: Subsystem Clock Supply Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osmcr`]
module"]
#[doc(alias = "OSMCR")]
pub type Osmcr = crate::Reg<osmcr::OsmcrSpec>;
#[doc = "Subsystem Clock Supply Mode Control Register"]
pub mod osmcr;
#[doc = "MOCOUTCR (rw) register accessor: MOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mocoutcr`]
module"]
#[doc(alias = "MOCOUTCR")]
pub type Mocoutcr = crate::Reg<mocoutcr::MocoutcrSpec>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: HOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hocoutcr`]
module"]
#[doc(alias = "HOCOUTCR")]
pub type Hocoutcr = crate::Reg<hocoutcr::HocoutcrSpec>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "SNZCR (rw) register accessor: Snooze Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzcr`]
module"]
#[doc(alias = "SNZCR")]
pub type Snzcr = crate::Reg<snzcr::SnzcrSpec>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR0 (rw) register accessor: Snooze End Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzedcr0`]
module"]
#[doc(alias = "SNZEDCR0")]
pub type Snzedcr0 = crate::Reg<snzedcr0::Snzedcr0Spec>;
#[doc = "Snooze End Control Register 0"]
pub mod snzedcr0;
#[doc = "SNZEDCR1 (rw) register accessor: Snooze End Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzedcr1`]
module"]
#[doc(alias = "SNZEDCR1")]
pub type Snzedcr1 = crate::Reg<snzedcr1::Snzedcr1Spec>;
#[doc = "Snooze End Control Register 1"]
pub mod snzedcr1;
#[doc = "SNZREQCR0 (rw) register accessor: Snooze Request Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`snzreqcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzreqcr0`]
module"]
#[doc(alias = "SNZREQCR0")]
pub type Snzreqcr0 = crate::Reg<snzreqcr0::Snzreqcr0Spec>;
#[doc = "Snooze Request Control Register 0"]
pub mod snzreqcr0;
#[doc = "PSMCR (rw) register accessor: Power Save Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmcr`]
module"]
#[doc(alias = "PSMCR")]
pub type Psmcr = crate::Reg<psmcr::PsmcrSpec>;
#[doc = "Power Save Memory Control Register"]
pub mod psmcr;
#[doc = "OPCCR (rw) register accessor: Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opccr`]
module"]
#[doc(alias = "OPCCR")]
pub type Opccr = crate::Reg<opccr::OpccrSpec>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "SOPCCR (rw) register accessor: Sub Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sopccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopccr`]
module"]
#[doc(alias = "SOPCCR")]
pub type Sopccr = crate::Reg<sopccr::SopccrSpec>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "RSTSR1 (rw) register accessor: Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr1`]
module"]
#[doc(alias = "RSTSR1")]
pub type Rstsr1 = crate::Reg<rstsr1::Rstsr1Spec>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
#[doc = "LVD1CR1 (rw) register accessor: Voltage Monitor 1 Circuit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd1cr1`]
module"]
#[doc(alias = "LVD1CR1")]
pub type Lvd1cr1 = crate::Reg<lvd1cr1::Lvd1cr1Spec>;
#[doc = "Voltage Monitor 1 Circuit Control Register"]
pub mod lvd1cr1;
#[doc = "LVD1SR (rw) register accessor: Voltage Monitor 1 Circuit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd1sr`]
module"]
#[doc(alias = "LVD1SR")]
pub type Lvd1sr = crate::Reg<lvd1sr::Lvd1srSpec>;
#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub mod lvd1sr;
#[doc = "LVD2CR1 (rw) register accessor: Voltage Monitor 2 Circuit Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd2cr1`]
module"]
#[doc(alias = "LVD2CR1")]
pub type Lvd2cr1 = crate::Reg<lvd2cr1::Lvd2cr1Spec>;
#[doc = "Voltage Monitor 2 Circuit Control Register 1"]
pub mod lvd2cr1;
#[doc = "LVD2SR (rw) register accessor: Voltage Monitor 2 Circuit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd2sr`]
module"]
#[doc(alias = "LVD2SR")]
pub type Lvd2sr = crate::Reg<lvd2sr::Lvd2srSpec>;
#[doc = "Voltage Monitor 2 Circuit Status Register"]
pub mod lvd2sr;
#[doc = "PRCR (rw) register accessor: Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prcr`]
module"]
#[doc(alias = "PRCR")]
pub type Prcr = crate::Reg<prcr::PrcrSpec>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "SYOCDCR (rw) register accessor: System Control OCD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syocdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syocdcr`]
module"]
#[doc(alias = "SYOCDCR")]
pub type Syocdcr = crate::Reg<syocdcr::SyocdcrSpec>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
#[doc = "RSTSR0 (rw) register accessor: Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr0`]
module"]
#[doc(alias = "RSTSR0")]
pub type Rstsr0 = crate::Reg<rstsr0::Rstsr0Spec>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: Reset Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr2`]
module"]
#[doc(alias = "RSTSR2")]
pub type Rstsr2 = crate::Reg<rstsr2::Rstsr2Spec>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "LVCMPCR (rw) register accessor: Voltage Monitor Circuit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvcmpcr`]
module"]
#[doc(alias = "LVCMPCR")]
pub type Lvcmpcr = crate::Reg<lvcmpcr::LvcmpcrSpec>;
#[doc = "Voltage Monitor Circuit Control Register"]
pub mod lvcmpcr;
#[doc = "LVDLVLR (rw) register accessor: Voltage Detection Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdlvlr`]
module"]
#[doc(alias = "LVDLVLR")]
pub type Lvdlvlr = crate::Reg<lvdlvlr::LvdlvlrSpec>;
#[doc = "Voltage Detection Level Select Register"]
pub mod lvdlvlr;
#[doc = "LVD1CR0 (rw) register accessor: Voltage Monitor 1 Circuit Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd1cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd1cr0`]
module"]
#[doc(alias = "LVD1CR0")]
pub type Lvd1cr0 = crate::Reg<lvd1cr0::Lvd1cr0Spec>;
#[doc = "Voltage Monitor 1 Circuit Control Register 0"]
pub mod lvd1cr0;
#[doc = "LVD2CR0 (rw) register accessor: Voltage Monitor 2 Circuit Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd2cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd2cr0`]
module"]
#[doc(alias = "LVD2CR0")]
pub type Lvd2cr0 = crate::Reg<lvd2cr0::Lvd2cr0Spec>;
#[doc = "Voltage Monitor 2 Circuit Control Register 0"]
pub mod lvd2cr0;
#[doc = "SOSCCR (rw) register accessor: Sub-Clock Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosccr`]
module"]
#[doc(alias = "SOSCCR")]
pub type Sosccr = crate::Reg<sosccr::SosccrSpec>;
#[doc = "Sub-Clock Oscillator Control Register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: Sub-Clock Oscillator Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`somcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@somcr`]
module"]
#[doc(alias = "SOMCR")]
pub type Somcr = crate::Reg<somcr::SomcrSpec>;
#[doc = "Sub-Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "SOMRG (rw) register accessor: Sub-Clock Oscillator Margin Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`somrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@somrg`]
module"]
#[doc(alias = "SOMRG")]
pub type Somrg = crate::Reg<somrg::SomrgSpec>;
#[doc = "Sub-Clock Oscillator Margin Check Register"]
pub mod somrg;
#[doc = "LOCOCR (rw) register accessor: Low-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lococr`]
module"]
#[doc(alias = "LOCOCR")]
pub type Lococr = crate::Reg<lococr::LococrSpec>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: LOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`locoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locoutcr`]
module"]
#[doc(alias = "LOCOUTCR")]
pub type Locoutcr = crate::Reg<locoutcr::LocoutcrSpec>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
