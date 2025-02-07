#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    remcon0: Remcon0,
    remcon1: Remcon1,
    remsts: Remsts,
    remint: Remint,
    _reserved4: [u8; 0x01],
    remcpc: Remcpc,
    remcpd: Remcpd,
    hdpmin: Hdpmin,
    hdpmax: Hdpmax,
    d0pmin: D0pmin,
    d0pmax: D0pmax,
    d1pmin: D1pmin,
    d1pmax: D1pmax,
    sdpmin: Sdpmin,
    sdpmax: Sdpmax,
    rempe: Rempe,
    remstc: Remstc,
    remrbit: Remrbit,
    remdat0: Remdat0,
    remdat: [Remdat; 7],
    remtim: Remtim,
}
impl RegisterBlock {
    #[doc = "0x00 - Function Select Register 0"]
    #[inline(always)]
    pub const fn remcon0(&self) -> &Remcon0 {
        &self.remcon0
    }
    #[doc = "0x01 - Function Select Register 1"]
    #[inline(always)]
    pub const fn remcon1(&self) -> &Remcon1 {
        &self.remcon1
    }
    #[doc = "0x02 - Status Register"]
    #[inline(always)]
    pub const fn remsts(&self) -> &Remsts {
        &self.remsts
    }
    #[doc = "0x03 - Interrupt Control Register"]
    #[inline(always)]
    pub const fn remint(&self) -> &Remint {
        &self.remint
    }
    #[doc = "0x05 - Compare Control Register"]
    #[inline(always)]
    pub const fn remcpc(&self) -> &Remcpc {
        &self.remcpc
    }
    #[doc = "0x06 - Compare Value Setting Register"]
    #[inline(always)]
    pub const fn remcpd(&self) -> &Remcpd {
        &self.remcpd
    }
    #[doc = "0x08 - Header Pattern Minimum Width Setting Register"]
    #[inline(always)]
    pub const fn hdpmin(&self) -> &Hdpmin {
        &self.hdpmin
    }
    #[doc = "0x0a - Header Pattern Maximum Width Setting Register"]
    #[inline(always)]
    pub const fn hdpmax(&self) -> &Hdpmax {
        &self.hdpmax
    }
    #[doc = "0x0c - Data 0 Pattern Minimum Width Setting Register"]
    #[inline(always)]
    pub const fn d0pmin(&self) -> &D0pmin {
        &self.d0pmin
    }
    #[doc = "0x0d - Data 0 Pattern Maximum Width Setting Register"]
    #[inline(always)]
    pub const fn d0pmax(&self) -> &D0pmax {
        &self.d0pmax
    }
    #[doc = "0x0e - Data 1 Pattern Minimum Width Setting Register"]
    #[inline(always)]
    pub const fn d1pmin(&self) -> &D1pmin {
        &self.d1pmin
    }
    #[doc = "0x0f - Data 1 Pattern Maximum Width Setting Register"]
    #[inline(always)]
    pub const fn d1pmax(&self) -> &D1pmax {
        &self.d1pmax
    }
    #[doc = "0x10 - Special Data Pattern Minimum Width Setting Register"]
    #[inline(always)]
    pub const fn sdpmin(&self) -> &Sdpmin {
        &self.sdpmin
    }
    #[doc = "0x12 - Special Data Pattern Maximum Width Setting Register"]
    #[inline(always)]
    pub const fn sdpmax(&self) -> &Sdpmax {
        &self.sdpmax
    }
    #[doc = "0x14 - Pattern End Setting Register"]
    #[inline(always)]
    pub const fn rempe(&self) -> &Rempe {
        &self.rempe
    }
    #[doc = "0x16 - Receiver Standby Control Register"]
    #[inline(always)]
    pub const fn remstc(&self) -> &Remstc {
        &self.remstc
    }
    #[doc = "0x17 - Receive Bit Count Register"]
    #[inline(always)]
    pub const fn remrbit(&self) -> &Remrbit {
        &self.remrbit
    }
    #[doc = "0x18 - Receive Data 0 Register"]
    #[inline(always)]
    pub const fn remdat0(&self) -> &Remdat0 {
        &self.remdat0
    }
    #[doc = "0x19..0x20 - Receive Data %s Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `REMDAT1` register.</div>"]
    #[inline(always)]
    pub const fn remdat(&self, n: usize) -> &Remdat {
        &self.remdat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x19..0x20 - Receive Data %s Register"]
    #[inline(always)]
    pub fn remdat_iter(&self) -> impl Iterator<Item = &Remdat> {
        self.remdat.iter()
    }
    #[doc = "0x19 - Receive Data 1 Register"]
    #[inline(always)]
    pub const fn remdat1(&self) -> &Remdat {
        self.remdat(0)
    }
    #[doc = "0x1a - Receive Data 2 Register"]
    #[inline(always)]
    pub const fn remdat2(&self) -> &Remdat {
        self.remdat(1)
    }
    #[doc = "0x1b - Receive Data 3 Register"]
    #[inline(always)]
    pub const fn remdat3(&self) -> &Remdat {
        self.remdat(2)
    }
    #[doc = "0x1c - Receive Data 4 Register"]
    #[inline(always)]
    pub const fn remdat4(&self) -> &Remdat {
        self.remdat(3)
    }
    #[doc = "0x1d - Receive Data 5 Register"]
    #[inline(always)]
    pub const fn remdat5(&self) -> &Remdat {
        self.remdat(4)
    }
    #[doc = "0x1e - Receive Data 6 Register"]
    #[inline(always)]
    pub const fn remdat6(&self) -> &Remdat {
        self.remdat(5)
    }
    #[doc = "0x1f - Receive Data 7 Register"]
    #[inline(always)]
    pub const fn remdat7(&self) -> &Remdat {
        self.remdat(6)
    }
    #[doc = "0x20 - Measurement Result Register"]
    #[inline(always)]
    pub const fn remtim(&self) -> &Remtim {
        &self.remtim
    }
}
#[doc = "REMCON0 (rw) register accessor: Function Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`remcon0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcon0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remcon0`]
module"]
#[doc(alias = "REMCON0")]
pub type Remcon0 = crate::Reg<remcon0::Remcon0Spec>;
#[doc = "Function Select Register 0"]
pub mod remcon0;
#[doc = "REMCON1 (rw) register accessor: Function Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`remcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remcon1`]
module"]
#[doc(alias = "REMCON1")]
pub type Remcon1 = crate::Reg<remcon1::Remcon1Spec>;
#[doc = "Function Select Register 1"]
pub mod remcon1;
#[doc = "REMSTS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remsts`]
module"]
#[doc(alias = "REMSTS")]
pub type Remsts = crate::Reg<remsts::RemstsSpec>;
#[doc = "Status Register"]
pub mod remsts;
#[doc = "REMINT (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remint`]
module"]
#[doc(alias = "REMINT")]
pub type Remint = crate::Reg<remint::RemintSpec>;
#[doc = "Interrupt Control Register"]
pub mod remint;
#[doc = "REMCPC (rw) register accessor: Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remcpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remcpc`]
module"]
#[doc(alias = "REMCPC")]
pub type Remcpc = crate::Reg<remcpc::RemcpcSpec>;
#[doc = "Compare Control Register"]
pub mod remcpc;
#[doc = "REMCPD (rw) register accessor: Compare Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remcpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remcpd`]
module"]
#[doc(alias = "REMCPD")]
pub type Remcpd = crate::Reg<remcpd::RemcpdSpec>;
#[doc = "Compare Value Setting Register"]
pub mod remcpd;
#[doc = "HDPMIN (rw) register accessor: Header Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdpmin`]
module"]
#[doc(alias = "HDPMIN")]
pub type Hdpmin = crate::Reg<hdpmin::HdpminSpec>;
#[doc = "Header Pattern Minimum Width Setting Register"]
pub mod hdpmin;
#[doc = "HDPMAX (rw) register accessor: Header Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdpmax`]
module"]
#[doc(alias = "HDPMAX")]
pub type Hdpmax = crate::Reg<hdpmax::HdpmaxSpec>;
#[doc = "Header Pattern Maximum Width Setting Register"]
pub mod hdpmax;
#[doc = "D0PMIN (rw) register accessor: Data 0 Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0pmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0pmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0pmin`]
module"]
#[doc(alias = "D0PMIN")]
pub type D0pmin = crate::Reg<d0pmin::D0pminSpec>;
#[doc = "Data 0 Pattern Minimum Width Setting Register"]
pub mod d0pmin;
#[doc = "D0PMAX (rw) register accessor: Data 0 Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0pmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0pmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0pmax`]
module"]
#[doc(alias = "D0PMAX")]
pub type D0pmax = crate::Reg<d0pmax::D0pmaxSpec>;
#[doc = "Data 0 Pattern Maximum Width Setting Register"]
pub mod d0pmax;
#[doc = "D1PMIN (rw) register accessor: Data 1 Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1pmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1pmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1pmin`]
module"]
#[doc(alias = "D1PMIN")]
pub type D1pmin = crate::Reg<d1pmin::D1pminSpec>;
#[doc = "Data 1 Pattern Minimum Width Setting Register"]
pub mod d1pmin;
#[doc = "D1PMAX (rw) register accessor: Data 1 Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1pmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1pmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1pmax`]
module"]
#[doc(alias = "D1PMAX")]
pub type D1pmax = crate::Reg<d1pmax::D1pmaxSpec>;
#[doc = "Data 1 Pattern Maximum Width Setting Register"]
pub mod d1pmax;
#[doc = "SDPMIN (rw) register accessor: Special Data Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdpmin`]
module"]
#[doc(alias = "SDPMIN")]
pub type Sdpmin = crate::Reg<sdpmin::SdpminSpec>;
#[doc = "Special Data Pattern Minimum Width Setting Register"]
pub mod sdpmin;
#[doc = "SDPMAX (rw) register accessor: Special Data Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdpmax`]
module"]
#[doc(alias = "SDPMAX")]
pub type Sdpmax = crate::Reg<sdpmax::SdpmaxSpec>;
#[doc = "Special Data Pattern Maximum Width Setting Register"]
pub mod sdpmax;
#[doc = "REMPE (rw) register accessor: Pattern End Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rempe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rempe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rempe`]
module"]
#[doc(alias = "REMPE")]
pub type Rempe = crate::Reg<rempe::RempeSpec>;
#[doc = "Pattern End Setting Register"]
pub mod rempe;
#[doc = "REMSTC (rw) register accessor: Receiver Standby Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remstc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remstc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remstc`]
module"]
#[doc(alias = "REMSTC")]
pub type Remstc = crate::Reg<remstc::RemstcSpec>;
#[doc = "Receiver Standby Control Register"]
pub mod remstc;
#[doc = "REMRBIT (rw) register accessor: Receive Bit Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remrbit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remrbit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remrbit`]
module"]
#[doc(alias = "REMRBIT")]
pub type Remrbit = crate::Reg<remrbit::RemrbitSpec>;
#[doc = "Receive Bit Count Register"]
pub mod remrbit;
#[doc = "REMDAT0 (rw) register accessor: Receive Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remdat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remdat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remdat0`]
module"]
#[doc(alias = "REMDAT0")]
pub type Remdat0 = crate::Reg<remdat0::Remdat0Spec>;
#[doc = "Receive Data 0 Register"]
pub mod remdat0;
#[doc = "REMDAT (r) register accessor: Receive Data %s Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remdat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remdat`]
module"]
#[doc(alias = "REMDAT")]
pub type Remdat = crate::Reg<remdat::RemdatSpec>;
#[doc = "Receive Data %s Register"]
pub mod remdat;
#[doc = "REMTIM (r) register accessor: Measurement Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remtim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remtim`]
module"]
#[doc(alias = "REMTIM")]
pub type Remtim = crate::Reg<remtim::RemtimSpec>;
#[doc = "Measurement Result Register"]
pub mod remtim;
