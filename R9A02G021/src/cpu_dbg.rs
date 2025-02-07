#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbgstr: Dbgstr,
    _reserved1: [u8; 0x0c],
    dbgstopcr: Dbgstopcr,
    _reserved2: [u8; 0x0fbc],
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Debug Status Register"]
    #[inline(always)]
    pub const fn dbgstr(&self) -> &Dbgstr {
        &self.dbgstr
    }
    #[doc = "0x10 - Debug Stop Control Register"]
    #[inline(always)]
    pub const fn dbgstopcr(&self) -> &Dbgstopcr {
        &self.dbgstopcr
    }
    #[doc = "0xfd0 - Peripheral Identification Register4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Peripheral Identification Register5"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Peripheral Identification Register6"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Peripheral Identification Register7"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Peripheral Identification Register0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Peripheral Identification Register1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Peripheral Identification Register2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Peripheral Identification Register3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Component Identification Register0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Component Identification Register1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Component Identification Register2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Component Identification Register3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "DBGSTR (r) register accessor: Debug Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstr`]
module"]
#[doc(alias = "DBGSTR")]
pub type Dbgstr = crate::Reg<dbgstr::DbgstrSpec>;
#[doc = "Debug Status Register"]
pub mod dbgstr;
#[doc = "DBGSTOPCR (rw) register accessor: Debug Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstopcr`]
module"]
#[doc(alias = "DBGSTOPCR")]
pub type Dbgstopcr = crate::Reg<dbgstopcr::DbgstopcrSpec>;
#[doc = "Debug Stop Control Register"]
pub mod dbgstopcr;
#[doc = "PIDR4 (r) register accessor: Peripheral Identification Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Peripheral Identification Register4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: Peripheral Identification Register5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`]
module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Peripheral Identification Register5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: Peripheral Identification Register6\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`]
module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Peripheral Identification Register6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: Peripheral Identification Register7\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`]
module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Peripheral Identification Register7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: Peripheral Identification Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Peripheral Identification Register0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: Peripheral Identification Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Peripheral Identification Register1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: Peripheral Identification Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Peripheral Identification Register2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: Peripheral Identification Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Peripheral Identification Register3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: Component Identification Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Component Identification Register0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: Component Identification Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Component Identification Register1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: Component Identification Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Component Identification Register2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: Component Identification Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Component Identification Register3"]
pub mod cidr3;
