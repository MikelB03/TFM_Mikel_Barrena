#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tdr00: Tdr00,
    _reserved_1_tdr0: [u8; 0x02],
    tdr02: Tdr02,
    _reserved_3_tdr0: [u8; 0x02],
    tdr04: Tdr04,
    tdr05: Tdr05,
    tdr06: Tdr06,
    tdr07: Tdr07,
    _reserved8: [u8; 0x70],
    tcr0: [Tcr0; 8],
    tmr00: Tmr00,
    tmr01: Tmr01,
    tmr02: Tmr02,
    tmr03: Tmr03,
    tmr04: Tmr04,
    tmr05: Tmr05,
    tmr06: Tmr06,
    tmr07: Tmr07,
    tsr0: [Tsr0; 8],
    te0: Te0,
    ts0: Ts0,
    tt0: Tt0,
    tps0: Tps0,
    to0: To0,
    toe0: Toe0,
    tol0: Tol0,
    tom0: Tom0,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Data Register 00"]
    #[inline(always)]
    pub const fn tdr00(&self) -> &Tdr00 {
        &self.tdr00
    }
    #[doc = "0x02 - Timer Data Register 01"]
    #[inline(always)]
    pub const fn tdr01l(&self) -> &Tdr01l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Timer Data Register 01"]
    #[inline(always)]
    pub const fn tdr01(&self) -> &Tdr01 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - Timer Data Register 01"]
    #[inline(always)]
    pub const fn tdr01h(&self) -> &Tdr01h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(3).cast() }
    }
    #[doc = "0x04 - Timer Data Register 02"]
    #[inline(always)]
    pub const fn tdr02(&self) -> &Tdr02 {
        &self.tdr02
    }
    #[doc = "0x06 - Timer Data Register 03"]
    #[inline(always)]
    pub const fn tdr03l(&self) -> &Tdr03l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - Timer Data Register 03"]
    #[inline(always)]
    pub const fn tdr03(&self) -> &Tdr03 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - Timer Data Register 03"]
    #[inline(always)]
    pub const fn tdr03h(&self) -> &Tdr03h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x08 - Timer Data Register 04"]
    #[inline(always)]
    pub const fn tdr04(&self) -> &Tdr04 {
        &self.tdr04
    }
    #[doc = "0x0a - Timer Data Register 05"]
    #[inline(always)]
    pub const fn tdr05(&self) -> &Tdr05 {
        &self.tdr05
    }
    #[doc = "0x0c - Timer Data Register 06"]
    #[inline(always)]
    pub const fn tdr06(&self) -> &Tdr06 {
        &self.tdr06
    }
    #[doc = "0x0e - Timer Data Register 07"]
    #[inline(always)]
    pub const fn tdr07(&self) -> &Tdr07 {
        &self.tdr07
    }
    #[doc = "0x80..0x90 - Timer Counter Register 0%s"]
    #[inline(always)]
    pub const fn tcr0(&self, n: usize) -> &Tcr0 {
        &self.tcr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Timer Counter Register 0%s"]
    #[inline(always)]
    pub fn tcr0_iter(&self) -> impl Iterator<Item = &Tcr0> {
        self.tcr0.iter()
    }
    #[doc = "0x90 - Timer Mode Register 00"]
    #[inline(always)]
    pub const fn tmr00(&self) -> &Tmr00 {
        &self.tmr00
    }
    #[doc = "0x92 - Timer Mode Register 01"]
    #[inline(always)]
    pub const fn tmr01(&self) -> &Tmr01 {
        &self.tmr01
    }
    #[doc = "0x94 - Timer Mode Register 02"]
    #[inline(always)]
    pub const fn tmr02(&self) -> &Tmr02 {
        &self.tmr02
    }
    #[doc = "0x96 - Timer Mode Register 03"]
    #[inline(always)]
    pub const fn tmr03(&self) -> &Tmr03 {
        &self.tmr03
    }
    #[doc = "0x98 - Timer Mode Register 04"]
    #[inline(always)]
    pub const fn tmr04(&self) -> &Tmr04 {
        &self.tmr04
    }
    #[doc = "0x9a - Timer Mode Register 05"]
    #[inline(always)]
    pub const fn tmr05(&self) -> &Tmr05 {
        &self.tmr05
    }
    #[doc = "0x9c - Timer Mode Register 06"]
    #[inline(always)]
    pub const fn tmr06(&self) -> &Tmr06 {
        &self.tmr06
    }
    #[doc = "0x9e - Timer Mode Register 07"]
    #[inline(always)]
    pub const fn tmr07(&self) -> &Tmr07 {
        &self.tmr07
    }
    #[doc = "0xa0..0xb0 - Timer Status Register 0%s"]
    #[inline(always)]
    pub const fn tsr0(&self, n: usize) -> &Tsr0 {
        &self.tsr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - Timer Status Register 0%s"]
    #[inline(always)]
    pub fn tsr0_iter(&self) -> impl Iterator<Item = &Tsr0> {
        self.tsr0.iter()
    }
    #[doc = "0xb0 - Timer Channel Enable Status Register 0"]
    #[inline(always)]
    pub const fn te0(&self) -> &Te0 {
        &self.te0
    }
    #[doc = "0xb2 - Timer Channel Start Register 0"]
    #[inline(always)]
    pub const fn ts0(&self) -> &Ts0 {
        &self.ts0
    }
    #[doc = "0xb4 - Timer Channel Stop Register 0"]
    #[inline(always)]
    pub const fn tt0(&self) -> &Tt0 {
        &self.tt0
    }
    #[doc = "0xb6 - Timer Clock Select Register 0"]
    #[inline(always)]
    pub const fn tps0(&self) -> &Tps0 {
        &self.tps0
    }
    #[doc = "0xb8 - Timer Output Register 0"]
    #[inline(always)]
    pub const fn to0(&self) -> &To0 {
        &self.to0
    }
    #[doc = "0xba - Timer Output Enable Register 0"]
    #[inline(always)]
    pub const fn toe0(&self) -> &Toe0 {
        &self.toe0
    }
    #[doc = "0xbc - Timer Output Level Register 0"]
    #[inline(always)]
    pub const fn tol0(&self) -> &Tol0 {
        &self.tol0
    }
    #[doc = "0xbe - Timer Output Mode Register 0"]
    #[inline(always)]
    pub const fn tom0(&self) -> &Tom0 {
        &self.tom0
    }
}
#[doc = "TDR00 (rw) register accessor: Timer Data Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr00`]
module"]
#[doc(alias = "TDR00")]
pub type Tdr00 = crate::Reg<tdr00::Tdr00Spec>;
#[doc = "Timer Data Register 00"]
pub mod tdr00;
#[doc = "TDR01 (rw) register accessor: Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr01`]
module"]
#[doc(alias = "TDR01")]
pub type Tdr01 = crate::Reg<tdr01::Tdr01Spec>;
#[doc = "Timer Data Register 01"]
pub mod tdr01;
#[doc = "TDR01L (rw) register accessor: Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr01l`]
module"]
#[doc(alias = "TDR01L")]
pub type Tdr01l = crate::Reg<tdr01l::Tdr01lSpec>;
#[doc = "Timer Data Register 01"]
pub mod tdr01l;
#[doc = "TDR01H (rw) register accessor: Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr01h`]
module"]
#[doc(alias = "TDR01H")]
pub type Tdr01h = crate::Reg<tdr01h::Tdr01hSpec>;
#[doc = "Timer Data Register 01"]
pub mod tdr01h;
#[doc = "TDR02 (rw) register accessor: Timer Data Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr02`]
module"]
#[doc(alias = "TDR02")]
pub type Tdr02 = crate::Reg<tdr02::Tdr02Spec>;
#[doc = "Timer Data Register 02"]
pub mod tdr02;
#[doc = "TDR03 (rw) register accessor: Timer Data Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr03`]
module"]
#[doc(alias = "TDR03")]
pub type Tdr03 = crate::Reg<tdr03::Tdr03Spec>;
#[doc = "Timer Data Register 03"]
pub mod tdr03;
#[doc = "TDR03L (rw) register accessor: Timer Data Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr03l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr03l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr03l`]
module"]
#[doc(alias = "TDR03L")]
pub type Tdr03l = crate::Reg<tdr03l::Tdr03lSpec>;
#[doc = "Timer Data Register 03"]
pub mod tdr03l;
#[doc = "TDR03H (rw) register accessor: Timer Data Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr03h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr03h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr03h`]
module"]
#[doc(alias = "TDR03H")]
pub type Tdr03h = crate::Reg<tdr03h::Tdr03hSpec>;
#[doc = "Timer Data Register 03"]
pub mod tdr03h;
#[doc = "TDR04 (rw) register accessor: Timer Data Register 04\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr04`]
module"]
#[doc(alias = "TDR04")]
pub type Tdr04 = crate::Reg<tdr04::Tdr04Spec>;
#[doc = "Timer Data Register 04"]
pub mod tdr04;
#[doc = "TDR05 (rw) register accessor: Timer Data Register 05\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr05`]
module"]
#[doc(alias = "TDR05")]
pub type Tdr05 = crate::Reg<tdr05::Tdr05Spec>;
#[doc = "Timer Data Register 05"]
pub mod tdr05;
#[doc = "TDR06 (rw) register accessor: Timer Data Register 06\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr06`]
module"]
#[doc(alias = "TDR06")]
pub type Tdr06 = crate::Reg<tdr06::Tdr06Spec>;
#[doc = "Timer Data Register 06"]
pub mod tdr06;
#[doc = "TDR07 (rw) register accessor: Timer Data Register 07\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr07`]
module"]
#[doc(alias = "TDR07")]
pub type Tdr07 = crate::Reg<tdr07::Tdr07Spec>;
#[doc = "Timer Data Register 07"]
pub mod tdr07;
#[doc = "TCR0 (r) register accessor: Timer Counter Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr0`]
module"]
#[doc(alias = "TCR0")]
pub type Tcr0 = crate::Reg<tcr0::Tcr0Spec>;
#[doc = "Timer Counter Register 0%s"]
pub mod tcr0;
#[doc = "TMR00 (rw) register accessor: Timer Mode Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr00`]
module"]
#[doc(alias = "TMR00")]
pub type Tmr00 = crate::Reg<tmr00::Tmr00Spec>;
#[doc = "Timer Mode Register 00"]
pub mod tmr00;
#[doc = "TMR01 (rw) register accessor: Timer Mode Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr01`]
module"]
#[doc(alias = "TMR01")]
pub type Tmr01 = crate::Reg<tmr01::Tmr01Spec>;
#[doc = "Timer Mode Register 01"]
pub mod tmr01;
#[doc = "TMR02 (rw) register accessor: Timer Mode Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr02`]
module"]
#[doc(alias = "TMR02")]
pub type Tmr02 = crate::Reg<tmr02::Tmr02Spec>;
#[doc = "Timer Mode Register 02"]
pub mod tmr02;
#[doc = "TMR03 (rw) register accessor: Timer Mode Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr03`]
module"]
#[doc(alias = "TMR03")]
pub type Tmr03 = crate::Reg<tmr03::Tmr03Spec>;
#[doc = "Timer Mode Register 03"]
pub mod tmr03;
#[doc = "TMR04 (rw) register accessor: Timer Mode Register 04\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr04`]
module"]
#[doc(alias = "TMR04")]
pub type Tmr04 = crate::Reg<tmr04::Tmr04Spec>;
#[doc = "Timer Mode Register 04"]
pub mod tmr04;
#[doc = "TMR05 (rw) register accessor: Timer Mode Register 05\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr05`]
module"]
#[doc(alias = "TMR05")]
pub type Tmr05 = crate::Reg<tmr05::Tmr05Spec>;
#[doc = "Timer Mode Register 05"]
pub mod tmr05;
#[doc = "TMR06 (rw) register accessor: Timer Mode Register 06\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr06`]
module"]
#[doc(alias = "TMR06")]
pub type Tmr06 = crate::Reg<tmr06::Tmr06Spec>;
#[doc = "Timer Mode Register 06"]
pub mod tmr06;
#[doc = "TMR07 (rw) register accessor: Timer Mode Register 07\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr07`]
module"]
#[doc(alias = "TMR07")]
pub type Tmr07 = crate::Reg<tmr07::Tmr07Spec>;
#[doc = "Timer Mode Register 07"]
pub mod tmr07;
#[doc = "TSR0 (r) register accessor: Timer Status Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr0`]
module"]
#[doc(alias = "TSR0")]
pub type Tsr0 = crate::Reg<tsr0::Tsr0Spec>;
#[doc = "Timer Status Register 0%s"]
pub mod tsr0;
#[doc = "TE0 (r) register accessor: Timer Channel Enable Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`te0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@te0`]
module"]
#[doc(alias = "TE0")]
pub type Te0 = crate::Reg<te0::Te0Spec>;
#[doc = "Timer Channel Enable Status Register 0"]
pub mod te0;
#[doc = "TS0 (rw) register accessor: Timer Channel Start Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts0`]
module"]
#[doc(alias = "TS0")]
pub type Ts0 = crate::Reg<ts0::Ts0Spec>;
#[doc = "Timer Channel Start Register 0"]
pub mod ts0;
#[doc = "TT0 (rw) register accessor: Timer Channel Stop Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tt0`]
module"]
#[doc(alias = "TT0")]
pub type Tt0 = crate::Reg<tt0::Tt0Spec>;
#[doc = "Timer Channel Stop Register 0"]
pub mod tt0;
#[doc = "TPS0 (rw) register accessor: Timer Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tps0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tps0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tps0`]
module"]
#[doc(alias = "TPS0")]
pub type Tps0 = crate::Reg<tps0::Tps0Spec>;
#[doc = "Timer Clock Select Register 0"]
pub mod tps0;
#[doc = "TO0 (rw) register accessor: Timer Output Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to0`]
module"]
#[doc(alias = "TO0")]
pub type To0 = crate::Reg<to0::To0Spec>;
#[doc = "Timer Output Register 0"]
pub mod to0;
#[doc = "TOE0 (rw) register accessor: Timer Output Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`toe0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toe0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toe0`]
module"]
#[doc(alias = "TOE0")]
pub type Toe0 = crate::Reg<toe0::Toe0Spec>;
#[doc = "Timer Output Enable Register 0"]
pub mod toe0;
#[doc = "TOL0 (rw) register accessor: Timer Output Level Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tol0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tol0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tol0`]
module"]
#[doc(alias = "TOL0")]
pub type Tol0 = crate::Reg<tol0::Tol0Spec>;
#[doc = "Timer Output Level Register 0"]
pub mod tol0;
#[doc = "TOM0 (rw) register accessor: Timer Output Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tom0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tom0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tom0`]
module"]
#[doc(alias = "TOM0")]
pub type Tom0 = crate::Reg<tom0::Tom0Spec>;
#[doc = "Timer Output Mode Register 0"]
pub mod tom0;
