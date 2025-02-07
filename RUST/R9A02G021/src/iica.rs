#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iica: (),
    _reserved1: [u8; 0x01],
    iics: (),
    _reserved2: [u8; 0x01],
    iicf: (),
    _reserved3: [u8; 0x7e],
    iicctl0: (),
    _reserved4: [u8; 0x01],
    iicctl1: (),
    _reserved5: [u8; 0x01],
    iicwl: (),
    _reserved6: [u8; 0x01],
    iicwh: (),
    _reserved7: [u8; 0x01],
    sva: (),
}
impl RegisterBlock {
    #[doc = "0x00 - IICA Shift Register %s"]
    #[inline(always)]
    pub const fn iica(&self, n: usize) -> &Iica {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - IICA Shift Register %s"]
    #[inline(always)]
    pub fn iica_iter(&self) -> impl Iterator<Item = &Iica> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() })
    }
    #[doc = "0x01 - IICA Status Register %s"]
    #[inline(always)]
    pub const fn iics(&self, n: usize) -> &Iics {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x01 - IICA Status Register %s"]
    #[inline(always)]
    pub fn iics_iter(&self) -> impl Iterator<Item = &Iics> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x02 - IICA Flag Register %s"]
    #[inline(always)]
    pub const fn iicf(&self, n: usize) -> &Iicf {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02 - IICA Flag Register %s"]
    #[inline(always)]
    pub fn iicf_iter(&self) -> impl Iterator<Item = &Iicf> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x80 - IICA Control Register n0"]
    #[inline(always)]
    pub const fn iicctl0(&self, n: usize) -> &Iicctl0 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80 - IICA Control Register n0"]
    #[inline(always)]
    pub fn iicctl0_iter(&self) -> impl Iterator<Item = &Iicctl0> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x80 - IICA Control Register n0"]
    #[inline(always)]
    pub const fn iicctl00(&self) -> &Iicctl0 {
        self.iicctl0(0)
    }
    #[doc = "0x180 - IICA Control Register n0"]
    #[inline(always)]
    pub const fn iicctl10(&self) -> &Iicctl0 {
        self.iicctl0(1)
    }
    #[doc = "0x81 - IICA Control Register n1"]
    #[inline(always)]
    pub const fn iicctl1(&self, n: usize) -> &Iicctl1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(129)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x81 - IICA Control Register n1"]
    #[inline(always)]
    pub fn iicctl1_iter(&self) -> impl Iterator<Item = &Iicctl1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(129)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x81 - IICA Control Register n1"]
    #[inline(always)]
    pub const fn iicctl01(&self) -> &Iicctl1 {
        self.iicctl1(0)
    }
    #[doc = "0x181 - IICA Control Register n1"]
    #[inline(always)]
    pub const fn iicctl11(&self) -> &Iicctl1 {
        self.iicctl1(1)
    }
    #[doc = "0x82 - IICA Low-level Width Setting Register %s"]
    #[inline(always)]
    pub const fn iicwl(&self, n: usize) -> &Iicwl {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(130)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x82 - IICA Low-level Width Setting Register %s"]
    #[inline(always)]
    pub fn iicwl_iter(&self) -> impl Iterator<Item = &Iicwl> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(130)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x83 - IICA High-level Width Setting Register %s"]
    #[inline(always)]
    pub const fn iicwh(&self, n: usize) -> &Iicwh {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(131)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x83 - IICA High-level Width Setting Register %s"]
    #[inline(always)]
    pub fn iicwh_iter(&self) -> impl Iterator<Item = &Iicwh> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(131)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x84 - Slave Address Register %s"]
    #[inline(always)]
    pub const fn sva(&self, n: usize) -> &Sva {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84 - Slave Address Register %s"]
    #[inline(always)]
    pub fn sva_iter(&self) -> impl Iterator<Item = &Sva> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "IICA (rw) register accessor: IICA Shift Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iica::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iica::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iica`]
module"]
#[doc(alias = "IICA")]
pub type Iica = crate::Reg<iica::IicaSpec>;
#[doc = "IICA Shift Register %s"]
pub mod iica;
#[doc = "IICS (r) register accessor: IICA Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iics::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iics`]
module"]
#[doc(alias = "IICS")]
pub type Iics = crate::Reg<iics::IicsSpec>;
#[doc = "IICA Status Register %s"]
pub mod iics;
#[doc = "IICF (rw) register accessor: IICA Flag Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iicf`]
module"]
#[doc(alias = "IICF")]
pub type Iicf = crate::Reg<iicf::IicfSpec>;
#[doc = "IICA Flag Register %s"]
pub mod iicf;
#[doc = "IICCTL0 (rw) register accessor: IICA Control Register n0\n\nYou can [`read`](crate::Reg::read) this register and get [`iicctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iicctl0`]
module"]
#[doc(alias = "IICCTL0")]
pub type Iicctl0 = crate::Reg<iicctl0::Iicctl0Spec>;
#[doc = "IICA Control Register n0"]
pub mod iicctl0;
#[doc = "IICCTL1 (rw) register accessor: IICA Control Register n1\n\nYou can [`read`](crate::Reg::read) this register and get [`iicctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iicctl1`]
module"]
#[doc(alias = "IICCTL1")]
pub type Iicctl1 = crate::Reg<iicctl1::Iicctl1Spec>;
#[doc = "IICA Control Register n1"]
pub mod iicctl1;
#[doc = "IICWL (rw) register accessor: IICA Low-level Width Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iicwl`]
module"]
#[doc(alias = "IICWL")]
pub type Iicwl = crate::Reg<iicwl::IicwlSpec>;
#[doc = "IICA Low-level Width Setting Register %s"]
pub mod iicwl;
#[doc = "IICWH (rw) register accessor: IICA High-level Width Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iicwh`]
module"]
#[doc(alias = "IICWH")]
pub type Iicwh = crate::Reg<iicwh::IicwhSpec>;
#[doc = "IICA High-level Width Setting Register %s"]
pub mod iicwh;
#[doc = "SVA (rw) register accessor: Slave Address Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`sva::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sva::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sva`]
module"]
#[doc(alias = "SVA")]
pub type Sva = crate::Reg<sva::SvaSpec>;
#[doc = "Slave Address Register %s"]
pub mod sva;
