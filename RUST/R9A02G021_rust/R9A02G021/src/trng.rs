#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    trngsdr: Trngsdr,
    _reserved1: [u8; 0x01],
    trngscr0: Trngscr0,
    trngscr1: Trngscr1,
}
impl RegisterBlock {
    #[doc = "0x00 - Random Number Seed Data Register"]
    #[inline(always)]
    pub const fn trngsdr(&self) -> &Trngsdr {
        &self.trngsdr
    }
    #[doc = "0x02 - Random Number Seed Command Register 0"]
    #[inline(always)]
    pub const fn trngscr0(&self) -> &Trngscr0 {
        &self.trngscr0
    }
    #[doc = "0x03 - Random Number Seed Command Register 1"]
    #[inline(always)]
    pub const fn trngscr1(&self) -> &Trngscr1 {
        &self.trngscr1
    }
}
#[doc = "TRNGSDR (r) register accessor: Random Number Seed Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trngsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngsdr`]
module"]
#[doc(alias = "TRNGSDR")]
pub type Trngsdr = crate::Reg<trngsdr::TrngsdrSpec>;
#[doc = "Random Number Seed Data Register"]
pub mod trngsdr;
#[doc = "TRNGSCR0 (rw) register accessor: Random Number Seed Command Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`trngscr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngscr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngscr0`]
module"]
#[doc(alias = "TRNGSCR0")]
pub type Trngscr0 = crate::Reg<trngscr0::Trngscr0Spec>;
#[doc = "Random Number Seed Command Register 0"]
pub mod trngscr0;
#[doc = "TRNGSCR1 (rw) register accessor: Random Number Seed Command Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`trngscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngscr1`]
module"]
#[doc(alias = "TRNGSCR1")]
pub type Trngscr1 = crate::Reg<trngscr1::Trngscr1Spec>;
#[doc = "Random Number Seed Command Register 1"]
pub mod trngscr1;
