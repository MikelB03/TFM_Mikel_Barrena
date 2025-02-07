#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdr1: [Sdr1; 2],
    _reserved1: [u8; 0x7c],
    ssr10: Ssr10,
    ssr11: Ssr11,
    _reserved3: [u8; 0x04],
    sir10: Sir10,
    sir11: Sir11,
    _reserved5: [u8; 0x04],
    smr10: Smr10,
    smr11: Smr11,
    _reserved7: [u8; 0x04],
    scr10: Scr10,
    scr11: Scr11,
    _reserved9: [u8; 0x04],
    se1: Se1,
    ss1: Ss1,
    st1: St1,
    sps1: Sps1,
    so1: So1,
    soe1: Soe1,
    _reserved15: [u8; 0x08],
    sol1: Sol1,
    _reserved16: [u8; 0x02],
    ssc1: Ssc1,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Data Register 1%s"]
    #[inline(always)]
    pub const fn sdr1(&self, n: usize) -> &Sdr1 {
        &self.sdr1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Serial Data Register 1%s"]
    #[inline(always)]
    pub fn sdr1_iter(&self) -> impl Iterator<Item = &Sdr1> {
        self.sdr1.iter()
    }
    #[doc = "0x80 - Serial Status Register 10"]
    #[inline(always)]
    pub const fn ssr10(&self) -> &Ssr10 {
        &self.ssr10
    }
    #[doc = "0x82 - Serial Status Register 11"]
    #[inline(always)]
    pub const fn ssr11(&self) -> &Ssr11 {
        &self.ssr11
    }
    #[doc = "0x88 - Serial Flag Clear Trigger Register 10"]
    #[inline(always)]
    pub const fn sir10(&self) -> &Sir10 {
        &self.sir10
    }
    #[doc = "0x8a - Serial Flag Clear Trigger Register 11"]
    #[inline(always)]
    pub const fn sir11(&self) -> &Sir11 {
        &self.sir11
    }
    #[doc = "0x90 - Serial Mode Register 10"]
    #[inline(always)]
    pub const fn smr10(&self) -> &Smr10 {
        &self.smr10
    }
    #[doc = "0x92 - Serial Mode Register 11"]
    #[inline(always)]
    pub const fn smr11(&self) -> &Smr11 {
        &self.smr11
    }
    #[doc = "0x98 - Serial Communication Operation Setting Register 10"]
    #[inline(always)]
    pub const fn scr10(&self) -> &Scr10 {
        &self.scr10
    }
    #[doc = "0x9a - Serial Communication Operation Setting Register 11"]
    #[inline(always)]
    pub const fn scr11(&self) -> &Scr11 {
        &self.scr11
    }
    #[doc = "0xa0 - Serial Channel Enable Status Register 1"]
    #[inline(always)]
    pub const fn se1(&self) -> &Se1 {
        &self.se1
    }
    #[doc = "0xa2 - Serial Channel Start Register 1"]
    #[inline(always)]
    pub const fn ss1(&self) -> &Ss1 {
        &self.ss1
    }
    #[doc = "0xa4 - Serial Channel Stop Register 1"]
    #[inline(always)]
    pub const fn st1(&self) -> &St1 {
        &self.st1
    }
    #[doc = "0xa6 - Serial Clock Select Register 1"]
    #[inline(always)]
    pub const fn sps1(&self) -> &Sps1 {
        &self.sps1
    }
    #[doc = "0xa8 - Serial Output Register 1"]
    #[inline(always)]
    pub const fn so1(&self) -> &So1 {
        &self.so1
    }
    #[doc = "0xaa - Serial Output Enable Register 1"]
    #[inline(always)]
    pub const fn soe1(&self) -> &Soe1 {
        &self.soe1
    }
    #[doc = "0xb4 - Serial Output Level Register 1"]
    #[inline(always)]
    pub const fn sol1(&self) -> &Sol1 {
        &self.sol1
    }
    #[doc = "0xb8 - Serial Standby Control Register 1"]
    #[inline(always)]
    pub const fn ssc1(&self) -> &Ssc1 {
        &self.ssc1
    }
}
#[doc = "SDR1 (rw) register accessor: Serial Data Register 1%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr1`]
module"]
#[doc(alias = "SDR1")]
pub type Sdr1 = crate::Reg<sdr1::Sdr1Spec>;
#[doc = "Serial Data Register 1%s"]
pub mod sdr1;
#[doc = "SSR10 (r) register accessor: Serial Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr10`]
module"]
#[doc(alias = "SSR10")]
pub type Ssr10 = crate::Reg<ssr10::Ssr10Spec>;
#[doc = "Serial Status Register 10"]
pub mod ssr10;
#[doc = "SSR11 (r) register accessor: Serial Status Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr11`]
module"]
#[doc(alias = "SSR11")]
pub type Ssr11 = crate::Reg<ssr11::Ssr11Spec>;
#[doc = "Serial Status Register 11"]
pub mod ssr11;
#[doc = "SIR10 (rw) register accessor: Serial Flag Clear Trigger Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`sir10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir10`]
module"]
#[doc(alias = "SIR10")]
pub type Sir10 = crate::Reg<sir10::Sir10Spec>;
#[doc = "Serial Flag Clear Trigger Register 10"]
pub mod sir10;
#[doc = "SIR11 (rw) register accessor: Serial Flag Clear Trigger Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`sir11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir11`]
module"]
#[doc(alias = "SIR11")]
pub type Sir11 = crate::Reg<sir11::Sir11Spec>;
#[doc = "Serial Flag Clear Trigger Register 11"]
pub mod sir11;
#[doc = "SMR10 (rw) register accessor: Serial Mode Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`smr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr10`]
module"]
#[doc(alias = "SMR10")]
pub type Smr10 = crate::Reg<smr10::Smr10Spec>;
#[doc = "Serial Mode Register 10"]
pub mod smr10;
#[doc = "SMR11 (rw) register accessor: Serial Mode Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`smr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr11`]
module"]
#[doc(alias = "SMR11")]
pub type Smr11 = crate::Reg<smr11::Smr11Spec>;
#[doc = "Serial Mode Register 11"]
pub mod smr11;
#[doc = "SCR10 (rw) register accessor: Serial Communication Operation Setting Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr10`]
module"]
#[doc(alias = "SCR10")]
pub type Scr10 = crate::Reg<scr10::Scr10Spec>;
#[doc = "Serial Communication Operation Setting Register 10"]
pub mod scr10;
#[doc = "SCR11 (rw) register accessor: Serial Communication Operation Setting Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr11`]
module"]
#[doc(alias = "SCR11")]
pub type Scr11 = crate::Reg<scr11::Scr11Spec>;
#[doc = "Serial Communication Operation Setting Register 11"]
pub mod scr11;
#[doc = "SE1 (r) register accessor: Serial Channel Enable Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`se1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se1`]
module"]
#[doc(alias = "SE1")]
pub type Se1 = crate::Reg<se1::Se1Spec>;
#[doc = "Serial Channel Enable Status Register 1"]
pub mod se1;
#[doc = "SS1 (rw) register accessor: Serial Channel Start Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ss1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss1`]
module"]
#[doc(alias = "SS1")]
pub type Ss1 = crate::Reg<ss1::Ss1Spec>;
#[doc = "Serial Channel Start Register 1"]
pub mod ss1;
#[doc = "ST1 (rw) register accessor: Serial Channel Stop Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`st1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1`]
module"]
#[doc(alias = "ST1")]
pub type St1 = crate::Reg<st1::St1Spec>;
#[doc = "Serial Channel Stop Register 1"]
pub mod st1;
#[doc = "SPS1 (rw) register accessor: Serial Clock Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sps1`]
module"]
#[doc(alias = "SPS1")]
pub type Sps1 = crate::Reg<sps1::Sps1Spec>;
#[doc = "Serial Clock Select Register 1"]
pub mod sps1;
#[doc = "SO1 (rw) register accessor: Serial Output Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`so1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`so1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@so1`]
module"]
#[doc(alias = "SO1")]
pub type So1 = crate::Reg<so1::So1Spec>;
#[doc = "Serial Output Register 1"]
pub mod so1;
#[doc = "SOE1 (rw) register accessor: Serial Output Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`soe1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soe1`]
module"]
#[doc(alias = "SOE1")]
pub type Soe1 = crate::Reg<soe1::Soe1Spec>;
#[doc = "Serial Output Enable Register 1"]
pub mod soe1;
#[doc = "SOL1 (rw) register accessor: Serial Output Level Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sol1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sol1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sol1`]
module"]
#[doc(alias = "SOL1")]
pub type Sol1 = crate::Reg<sol1::Sol1Spec>;
#[doc = "Serial Output Level Register 1"]
pub mod sol1;
#[doc = "SSC1 (rw) register accessor: Serial Standby Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ssc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssc1`]
module"]
#[doc(alias = "SSC1")]
pub type Ssc1 = crate::Reg<ssc1::Ssc1Spec>;
#[doc = "Serial Standby Control Register 1"]
pub mod ssc1;
