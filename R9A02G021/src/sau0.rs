#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdr0: [Sdr0; 4],
    _reserved1: [u8; 0x78],
    ssr00: Ssr00,
    ssr01: Ssr01,
    ssr02: Ssr02,
    ssr03: Ssr03,
    sir00: Sir00,
    sir01: Sir01,
    sir02: Sir02,
    sir03: Sir03,
    smr00: Smr00,
    smr01: Smr01,
    smr02: Smr02,
    smr03: Smr03,
    scr00: Scr00,
    scr01: Scr01,
    scr02: Scr02,
    scr03: Scr03,
    se0: Se0,
    ss0: Ss0,
    st0: St0,
    sps0: Sps0,
    so0: So0,
    soe0: Soe0,
    _reserved23: [u8; 0x08],
    sol0: Sol0,
    _reserved24: [u8; 0x02],
    ssc0: Ssc0,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Serial Data Register 0%s"]
    #[inline(always)]
    pub const fn sdr0(&self, n: usize) -> &Sdr0 {
        &self.sdr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Serial Data Register 0%s"]
    #[inline(always)]
    pub fn sdr0_iter(&self) -> impl Iterator<Item = &Sdr0> {
        self.sdr0.iter()
    }
    #[doc = "0x80 - Serial Status Register 00"]
    #[inline(always)]
    pub const fn ssr00(&self) -> &Ssr00 {
        &self.ssr00
    }
    #[doc = "0x82 - Serial Status Register 01"]
    #[inline(always)]
    pub const fn ssr01(&self) -> &Ssr01 {
        &self.ssr01
    }
    #[doc = "0x84 - Serial Status Register 02"]
    #[inline(always)]
    pub const fn ssr02(&self) -> &Ssr02 {
        &self.ssr02
    }
    #[doc = "0x86 - Serial Status Register 03"]
    #[inline(always)]
    pub const fn ssr03(&self) -> &Ssr03 {
        &self.ssr03
    }
    #[doc = "0x88 - Serial Flag Clear Trigger Register 00"]
    #[inline(always)]
    pub const fn sir00(&self) -> &Sir00 {
        &self.sir00
    }
    #[doc = "0x8a - Serial Flag Clear Trigger Register 01"]
    #[inline(always)]
    pub const fn sir01(&self) -> &Sir01 {
        &self.sir01
    }
    #[doc = "0x8c - Serial Flag Clear Trigger Register 02"]
    #[inline(always)]
    pub const fn sir02(&self) -> &Sir02 {
        &self.sir02
    }
    #[doc = "0x8e - Serial Flag Clear Trigger Register 03"]
    #[inline(always)]
    pub const fn sir03(&self) -> &Sir03 {
        &self.sir03
    }
    #[doc = "0x90 - Serial Mode Register 00"]
    #[inline(always)]
    pub const fn smr00(&self) -> &Smr00 {
        &self.smr00
    }
    #[doc = "0x92 - Serial Mode Register 01"]
    #[inline(always)]
    pub const fn smr01(&self) -> &Smr01 {
        &self.smr01
    }
    #[doc = "0x94 - Serial Mode Register 02"]
    #[inline(always)]
    pub const fn smr02(&self) -> &Smr02 {
        &self.smr02
    }
    #[doc = "0x96 - Serial Mode Register 03"]
    #[inline(always)]
    pub const fn smr03(&self) -> &Smr03 {
        &self.smr03
    }
    #[doc = "0x98 - Serial Communication Operation Setting Register 00"]
    #[inline(always)]
    pub const fn scr00(&self) -> &Scr00 {
        &self.scr00
    }
    #[doc = "0x9a - Serial Communication Operation Setting Register 01"]
    #[inline(always)]
    pub const fn scr01(&self) -> &Scr01 {
        &self.scr01
    }
    #[doc = "0x9c - Serial Communication Operation Setting Register 02"]
    #[inline(always)]
    pub const fn scr02(&self) -> &Scr02 {
        &self.scr02
    }
    #[doc = "0x9e - Serial Communication Operation Setting Register 03"]
    #[inline(always)]
    pub const fn scr03(&self) -> &Scr03 {
        &self.scr03
    }
    #[doc = "0xa0 - Serial Channel Enable Status Register 0"]
    #[inline(always)]
    pub const fn se0(&self) -> &Se0 {
        &self.se0
    }
    #[doc = "0xa2 - Serial Channel Start Register 0"]
    #[inline(always)]
    pub const fn ss0(&self) -> &Ss0 {
        &self.ss0
    }
    #[doc = "0xa4 - Serial Channel Stop Register 0"]
    #[inline(always)]
    pub const fn st0(&self) -> &St0 {
        &self.st0
    }
    #[doc = "0xa6 - Serial Clock Select Register 0"]
    #[inline(always)]
    pub const fn sps0(&self) -> &Sps0 {
        &self.sps0
    }
    #[doc = "0xa8 - Serial Output Register 0"]
    #[inline(always)]
    pub const fn so0(&self) -> &So0 {
        &self.so0
    }
    #[doc = "0xaa - Serial Output Enable Register 0"]
    #[inline(always)]
    pub const fn soe0(&self) -> &Soe0 {
        &self.soe0
    }
    #[doc = "0xb4 - Serial Output Level Register 0"]
    #[inline(always)]
    pub const fn sol0(&self) -> &Sol0 {
        &self.sol0
    }
    #[doc = "0xb8 - Serial Standby Control Register 0"]
    #[inline(always)]
    pub const fn ssc0(&self) -> &Ssc0 {
        &self.ssc0
    }
}
#[doc = "SDR0 (rw) register accessor: Serial Data Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr0`]
module"]
#[doc(alias = "SDR0")]
pub type Sdr0 = crate::Reg<sdr0::Sdr0Spec>;
#[doc = "Serial Data Register 0%s"]
pub mod sdr0;
#[doc = "SSR00 (r) register accessor: Serial Status Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr00::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr00`]
module"]
#[doc(alias = "SSR00")]
pub type Ssr00 = crate::Reg<ssr00::Ssr00Spec>;
#[doc = "Serial Status Register 00"]
pub mod ssr00;
#[doc = "SSR01 (r) register accessor: Serial Status Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr01`]
module"]
#[doc(alias = "SSR01")]
pub type Ssr01 = crate::Reg<ssr01::Ssr01Spec>;
#[doc = "Serial Status Register 01"]
pub mod ssr01;
#[doc = "SSR02 (r) register accessor: Serial Status Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr02::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr02`]
module"]
#[doc(alias = "SSR02")]
pub type Ssr02 = crate::Reg<ssr02::Ssr02Spec>;
#[doc = "Serial Status Register 02"]
pub mod ssr02;
#[doc = "SSR03 (r) register accessor: Serial Status Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr03::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr03`]
module"]
#[doc(alias = "SSR03")]
pub type Ssr03 = crate::Reg<ssr03::Ssr03Spec>;
#[doc = "Serial Status Register 03"]
pub mod ssr03;
#[doc = "SIR00 (rw) register accessor: Serial Flag Clear Trigger Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`sir00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir00`]
module"]
#[doc(alias = "SIR00")]
pub type Sir00 = crate::Reg<sir00::Sir00Spec>;
#[doc = "Serial Flag Clear Trigger Register 00"]
pub mod sir00;
#[doc = "SIR01 (rw) register accessor: Serial Flag Clear Trigger Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`sir01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir01`]
module"]
#[doc(alias = "SIR01")]
pub type Sir01 = crate::Reg<sir01::Sir01Spec>;
#[doc = "Serial Flag Clear Trigger Register 01"]
pub mod sir01;
#[doc = "SIR02 (rw) register accessor: Serial Flag Clear Trigger Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`sir02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir02`]
module"]
#[doc(alias = "SIR02")]
pub type Sir02 = crate::Reg<sir02::Sir02Spec>;
#[doc = "Serial Flag Clear Trigger Register 02"]
pub mod sir02;
#[doc = "SIR03 (rw) register accessor: Serial Flag Clear Trigger Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`sir03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sir03`]
module"]
#[doc(alias = "SIR03")]
pub type Sir03 = crate::Reg<sir03::Sir03Spec>;
#[doc = "Serial Flag Clear Trigger Register 03"]
pub mod sir03;
#[doc = "SMR00 (rw) register accessor: Serial Mode Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`smr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr00`]
module"]
#[doc(alias = "SMR00")]
pub type Smr00 = crate::Reg<smr00::Smr00Spec>;
#[doc = "Serial Mode Register 00"]
pub mod smr00;
#[doc = "SMR01 (rw) register accessor: Serial Mode Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`smr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr01`]
module"]
#[doc(alias = "SMR01")]
pub type Smr01 = crate::Reg<smr01::Smr01Spec>;
#[doc = "Serial Mode Register 01"]
pub mod smr01;
#[doc = "SMR02 (rw) register accessor: Serial Mode Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`smr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr02`]
module"]
#[doc(alias = "SMR02")]
pub type Smr02 = crate::Reg<smr02::Smr02Spec>;
#[doc = "Serial Mode Register 02"]
pub mod smr02;
#[doc = "SMR03 (rw) register accessor: Serial Mode Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`smr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr03`]
module"]
#[doc(alias = "SMR03")]
pub type Smr03 = crate::Reg<smr03::Smr03Spec>;
#[doc = "Serial Mode Register 03"]
pub mod smr03;
#[doc = "SCR00 (rw) register accessor: Serial Communication Operation Setting Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`scr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr00`]
module"]
#[doc(alias = "SCR00")]
pub type Scr00 = crate::Reg<scr00::Scr00Spec>;
#[doc = "Serial Communication Operation Setting Register 00"]
pub mod scr00;
#[doc = "SCR01 (rw) register accessor: Serial Communication Operation Setting Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`scr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr01`]
module"]
#[doc(alias = "SCR01")]
pub type Scr01 = crate::Reg<scr01::Scr01Spec>;
#[doc = "Serial Communication Operation Setting Register 01"]
pub mod scr01;
#[doc = "SCR02 (rw) register accessor: Serial Communication Operation Setting Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`scr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr02`]
module"]
#[doc(alias = "SCR02")]
pub type Scr02 = crate::Reg<scr02::Scr02Spec>;
#[doc = "Serial Communication Operation Setting Register 02"]
pub mod scr02;
#[doc = "SCR03 (rw) register accessor: Serial Communication Operation Setting Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`scr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr03`]
module"]
#[doc(alias = "SCR03")]
pub type Scr03 = crate::Reg<scr03::Scr03Spec>;
#[doc = "Serial Communication Operation Setting Register 03"]
pub mod scr03;
#[doc = "SE0 (r) register accessor: Serial Channel Enable Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se0`]
module"]
#[doc(alias = "SE0")]
pub type Se0 = crate::Reg<se0::Se0Spec>;
#[doc = "Serial Channel Enable Status Register 0"]
pub mod se0;
#[doc = "SS0 (rw) register accessor: Serial Channel Start Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ss0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss0`]
module"]
#[doc(alias = "SS0")]
pub type Ss0 = crate::Reg<ss0::Ss0Spec>;
#[doc = "Serial Channel Start Register 0"]
pub mod ss0;
#[doc = "ST0 (rw) register accessor: Serial Channel Stop Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`st0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0`]
module"]
#[doc(alias = "ST0")]
pub type St0 = crate::Reg<st0::St0Spec>;
#[doc = "Serial Channel Stop Register 0"]
pub mod st0;
#[doc = "SPS0 (rw) register accessor: Serial Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sps0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sps0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sps0`]
module"]
#[doc(alias = "SPS0")]
pub type Sps0 = crate::Reg<sps0::Sps0Spec>;
#[doc = "Serial Clock Select Register 0"]
pub mod sps0;
#[doc = "SO0 (rw) register accessor: Serial Output Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`so0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`so0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@so0`]
module"]
#[doc(alias = "SO0")]
pub type So0 = crate::Reg<so0::So0Spec>;
#[doc = "Serial Output Register 0"]
pub mod so0;
#[doc = "SOE0 (rw) register accessor: Serial Output Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`soe0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soe0`]
module"]
#[doc(alias = "SOE0")]
pub type Soe0 = crate::Reg<soe0::Soe0Spec>;
#[doc = "Serial Output Enable Register 0"]
pub mod soe0;
#[doc = "SOL0 (rw) register accessor: Serial Output Level Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sol0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sol0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sol0`]
module"]
#[doc(alias = "SOL0")]
pub type Sol0 = crate::Reg<sol0::Sol0Spec>;
#[doc = "Serial Output Level Register 0"]
pub mod sol0;
#[doc = "SSC0 (rw) register accessor: Serial Standby Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssc0`]
module"]
#[doc(alias = "SSC0")]
pub type Ssc0 = crate::Reg<ssc0::Ssc0Spec>;
#[doc = "Serial Standby Control Register 0"]
pub mod ssc0;
