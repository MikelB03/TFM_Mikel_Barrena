#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    iltmemctl: Iltmemctl,
    _reserved1: [u8; 0x03fe],
    busmcntinst: Busmcntinst,
    _reserved2: [u8; 0x02],
    busmcntdat: Busmcntdat,
    _reserved3: [u8; 0x02],
    busmcntdma: Busmcntdma,
    _reserved4: [u8; 0x03f6],
    buscntoad: Buscntoad,
    _reserved5: [u8; 0x03fe],
    bus1erradd: Bus1erradd,
    bus1errstat: Bus1errstat,
    _reserved7: [u8; 0x0b],
    bus2erradd: Bus2erradd,
    bus2errstat: Bus2errstat,
    _reserved9: [u8; 0x0b],
    bus3erradd: Bus3erradd,
    bus3errstat: Bus3errstat,
}
impl RegisterBlock {
    #[doc = "0xc00 - Illicit Memory Access Detection Control Register"]
    #[inline(always)]
    pub const fn iltmemctl(&self) -> &Iltmemctl {
        &self.iltmemctl
    }
    #[doc = "0x1000 - Control Register INST"]
    #[inline(always)]
    pub const fn busmcntinst(&self) -> &Busmcntinst {
        &self.busmcntinst
    }
    #[doc = "0x1004 - Control Register DAT"]
    #[inline(always)]
    pub const fn busmcntdat(&self) -> &Busmcntdat {
        &self.busmcntdat
    }
    #[doc = "0x1008 - Control Register DMA"]
    #[inline(always)]
    pub const fn busmcntdma(&self) -> &Busmcntdma {
        &self.busmcntdma
    }
    #[doc = "0x1400 - Bus Control Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn buscntoad(&self) -> &Buscntoad {
        &self.buscntoad
    }
    #[doc = "0x1800 - Bus Error Address Register 1"]
    #[inline(always)]
    pub const fn bus1erradd(&self) -> &Bus1erradd {
        &self.bus1erradd
    }
    #[doc = "0x1804 - BUS Error Status Register 1"]
    #[inline(always)]
    pub const fn bus1errstat(&self) -> &Bus1errstat {
        &self.bus1errstat
    }
    #[doc = "0x1810 - Bus Error Address Register 2"]
    #[inline(always)]
    pub const fn bus2erradd(&self) -> &Bus2erradd {
        &self.bus2erradd
    }
    #[doc = "0x1814 - BUS Error Status Register 2"]
    #[inline(always)]
    pub const fn bus2errstat(&self) -> &Bus2errstat {
        &self.bus2errstat
    }
    #[doc = "0x1820 - Bus Error Address Register 3"]
    #[inline(always)]
    pub const fn bus3erradd(&self) -> &Bus3erradd {
        &self.bus3erradd
    }
    #[doc = "0x1824 - BUS Error Status Register 3"]
    #[inline(always)]
    pub const fn bus3errstat(&self) -> &Bus3errstat {
        &self.bus3errstat
    }
}
#[doc = "ILTMEMCTL (rw) register accessor: Illicit Memory Access Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iltmemctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iltmemctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iltmemctl`]
module"]
#[doc(alias = "ILTMEMCTL")]
pub type Iltmemctl = crate::Reg<iltmemctl::IltmemctlSpec>;
#[doc = "Illicit Memory Access Detection Control Register"]
pub mod iltmemctl;
#[doc = "BUSMCNTINST (rw) register accessor: Control Register INST\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcntinst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntinst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmcntinst`]
module"]
#[doc(alias = "BUSMCNTINST")]
pub type Busmcntinst = crate::Reg<busmcntinst::BusmcntinstSpec>;
#[doc = "Control Register INST"]
pub mod busmcntinst;
#[doc = "BUSMCNTDAT (rw) register accessor: Control Register DAT\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcntdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmcntdat`]
module"]
#[doc(alias = "BUSMCNTDAT")]
pub type Busmcntdat = crate::Reg<busmcntdat::BusmcntdatSpec>;
#[doc = "Control Register DAT"]
pub mod busmcntdat;
#[doc = "BUSMCNTDMA (rw) register accessor: Control Register DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcntdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmcntdma`]
module"]
#[doc(alias = "BUSMCNTDMA")]
pub type Busmcntdma = crate::Reg<busmcntdma::BusmcntdmaSpec>;
#[doc = "Control Register DMA"]
pub mod busmcntdma;
#[doc = "BUSCNTOAD (rw) register accessor: Bus Control Error Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buscntoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buscntoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buscntoad`]
module"]
#[doc(alias = "BUSCNTOAD")]
pub type Buscntoad = crate::Reg<buscntoad::BuscntoadSpec>;
#[doc = "Bus Control Error Operation After Detection Register"]
pub mod buscntoad;
#[doc = "BUS1ERRADD (r) register accessor: Bus Error Address Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1erradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1erradd`]
module"]
#[doc(alias = "BUS1ERRADD")]
pub type Bus1erradd = crate::Reg<bus1erradd::Bus1erraddSpec>;
#[doc = "Bus Error Address Register 1"]
pub mod bus1erradd;
#[doc = "BUS1ERRSTAT (r) register accessor: BUS Error Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1errstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1errstat`]
module"]
#[doc(alias = "BUS1ERRSTAT")]
pub type Bus1errstat = crate::Reg<bus1errstat::Bus1errstatSpec>;
#[doc = "BUS Error Status Register 1"]
pub mod bus1errstat;
#[doc = "BUS2ERRADD (r) register accessor: Bus Error Address Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus2erradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus2erradd`]
module"]
#[doc(alias = "BUS2ERRADD")]
pub type Bus2erradd = crate::Reg<bus2erradd::Bus2erraddSpec>;
#[doc = "Bus Error Address Register 2"]
pub mod bus2erradd;
#[doc = "BUS2ERRSTAT (r) register accessor: BUS Error Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus2errstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus2errstat`]
module"]
#[doc(alias = "BUS2ERRSTAT")]
pub type Bus2errstat = crate::Reg<bus2errstat::Bus2errstatSpec>;
#[doc = "BUS Error Status Register 2"]
pub mod bus2errstat;
#[doc = "BUS3ERRADD (r) register accessor: Bus Error Address Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bus3erradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus3erradd`]
module"]
#[doc(alias = "BUS3ERRADD")]
pub type Bus3erradd = crate::Reg<bus3erradd::Bus3erraddSpec>;
#[doc = "Bus Error Address Register 3"]
pub mod bus3erradd;
#[doc = "BUS3ERRSTAT (r) register accessor: BUS Error Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bus3errstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus3errstat`]
module"]
#[doc(alias = "BUS3ERRSTAT")]
pub type Bus3errstat = crate::Reg<bus3errstat::Bus3errstatSpec>;
#[doc = "BUS Error Status Register 3"]
pub mod bus3errstat;
