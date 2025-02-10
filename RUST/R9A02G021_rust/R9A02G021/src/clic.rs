#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cliccfg: Cliccfg,
    _reserved1: [u8; 0x03],
    clicinfo: Clicinfo,
    _reserved2: [u8; 0x0ff8],
    clicintip: (),
    _reserved3: [u8; 0x01],
    clicintie: (),
    _reserved4: [u8; 0x01],
    clicintattr: (),
    _reserved5: [u8; 0x01],
    clicintctl: (),
}
impl RegisterBlock {
    #[doc = "0x00 - CLIC Configuration Register"]
    #[inline(always)]
    pub const fn cliccfg(&self) -> &Cliccfg {
        &self.cliccfg
    }
    #[doc = "0x04 - CLIC Information Register"]
    #[inline(always)]
    pub const fn clicinfo(&self) -> &Clicinfo {
        &self.clicinfo
    }
    #[doc = "0x1000..0x1033 - CLIC Interrupt Pending Register"]
    #[inline(always)]
    pub const fn clicintip(&self, n: usize) -> &Clicintip {
        #[allow(clippy::no_effect)]
        [(); 51][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1033 - CLIC Interrupt Pending Register"]
    #[inline(always)]
    pub fn clicintip_iter(&self) -> impl Iterator<Item = &Clicintip> {
        (0..51).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1001..0x1034 - CLIC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn clicintie(&self, n: usize) -> &Clicintie {
        #[allow(clippy::no_effect)]
        [(); 51][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4097)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1001..0x1034 - CLIC Interrupt Enable Register"]
    #[inline(always)]
    pub fn clicintie_iter(&self) -> impl Iterator<Item = &Clicintie> {
        (0..51).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4097)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1002..0x1035 - "]
    #[inline(always)]
    pub const fn clicintattr(&self, n: usize) -> &Clicintattr {
        #[allow(clippy::no_effect)]
        [(); 51][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4098)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1002..0x1035 - "]
    #[inline(always)]
    pub fn clicintattr_iter(&self) -> impl Iterator<Item = &Clicintattr> {
        (0..51).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4098)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1003..0x1036 - "]
    #[inline(always)]
    pub const fn clicintctl(&self, n: usize) -> &Clicintctl {
        #[allow(clippy::no_effect)]
        [(); 51][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4099)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1003..0x1036 - "]
    #[inline(always)]
    pub fn clicintctl_iter(&self) -> impl Iterator<Item = &Clicintctl> {
        (0..51).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4099)
                .add(4 * n)
                .cast()
        })
    }
}
#[doc = "cliccfg (rw) register accessor: CLIC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cliccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cliccfg`]
module"]
#[doc(alias = "cliccfg")]
pub type Cliccfg = crate::Reg<cliccfg::CliccfgSpec>;
#[doc = "CLIC Configuration Register"]
pub mod cliccfg;
#[doc = "clicinfo (r) register accessor: CLIC Information Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicinfo`]
module"]
#[doc(alias = "clicinfo")]
pub type Clicinfo = crate::Reg<clicinfo::ClicinfoSpec>;
#[doc = "CLIC Information Register"]
pub mod clicinfo;
#[doc = "clicintip (rw) register accessor: CLIC Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintip`]
module"]
#[doc(alias = "clicintip")]
pub type Clicintip = crate::Reg<clicintip::ClicintipSpec>;
#[doc = "CLIC Interrupt Pending Register"]
pub mod clicintip;
#[doc = "clicintie (rw) register accessor: CLIC Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintie`]
module"]
#[doc(alias = "clicintie")]
pub type Clicintie = crate::Reg<clicintie::ClicintieSpec>;
#[doc = "CLIC Interrupt Enable Register"]
pub mod clicintie;
#[doc = "clicintattr (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clicintattr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintattr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintattr`]
module"]
#[doc(alias = "clicintattr")]
pub type Clicintattr = crate::Reg<clicintattr::ClicintattrSpec>;
#[doc = ""]
pub mod clicintattr;
#[doc = "clicintctl (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clicintctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintctl`]
module"]
#[doc(alias = "clicintctl")]
pub type Clicintctl = crate::Reg<clicintctl::ClicintctlSpec>;
#[doc = ""]
pub mod clicintctl;
