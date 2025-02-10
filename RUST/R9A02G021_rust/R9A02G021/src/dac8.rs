#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dadr: [Dadr; 2],
    dacr: Dacr,
    dadpr: Dadpr,
    _reserved3: [u8; 0x06fa],
    daexout: Daexout,
}
impl RegisterBlock {
    #[doc = "0x00 - D/A Data Register %s"]
    #[inline(always)]
    pub const fn dadr(&self, n: usize) -> &Dadr {
        &self.dadr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - D/A Data Register %s"]
    #[inline(always)]
    pub fn dadr_iter(&self) -> impl Iterator<Item = &Dadr> {
        self.dadr.iter()
    }
    #[doc = "0x04 - D/A Control Register"]
    #[inline(always)]
    pub const fn dacr(&self) -> &Dacr {
        &self.dacr
    }
    #[doc = "0x05 - DADRn Format Select Register"]
    #[inline(always)]
    pub const fn dadpr(&self) -> &Dadpr {
        &self.dadpr
    }
    #[doc = "0x700 - D/A External Output Enable Register"]
    #[inline(always)]
    pub const fn daexout(&self) -> &Daexout {
        &self.daexout
    }
}
#[doc = "DADR (rw) register accessor: D/A Data Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr`]
module"]
#[doc(alias = "DADR")]
pub type Dadr = crate::Reg<dadr::DadrSpec>;
#[doc = "D/A Data Register %s"]
pub mod dadr;
#[doc = "DACR (rw) register accessor: D/A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacr`]
module"]
#[doc(alias = "DACR")]
pub type Dacr = crate::Reg<dacr::DacrSpec>;
#[doc = "D/A Control Register"]
pub mod dacr;
#[doc = "DADPR (rw) register accessor: DADRn Format Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dadpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadpr`]
module"]
#[doc(alias = "DADPR")]
pub type Dadpr = crate::Reg<dadpr::DadprSpec>;
#[doc = "DADRn Format Select Register"]
pub mod dadpr;
#[doc = "DAEXOUT (rw) register accessor: D/A External Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daexout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daexout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daexout`]
module"]
#[doc(alias = "DAEXOUT")]
pub type Daexout = crate::Reg<daexout::DaexoutSpec>;
#[doc = "D/A External Output Enable Register"]
pub mod daexout;
