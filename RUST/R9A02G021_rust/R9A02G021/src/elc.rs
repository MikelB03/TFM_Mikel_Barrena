#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    elcr: Elcr,
    _reserved1: [u8; 0x01],
    elsegr: (),
    _reserved2: [u8; 0x2e],
    elsr8: Elsr8,
    _reserved3: [u8; 0x16],
    elsr: (),
    _reserved4: [u8; 0x14],
    elsr19: Elsr,
    _reserved5: [u8; 0x02],
    elsr20: Elsr,
    _reserved6: [u8; 0x0a],
    elsr23: Elsr23,
}
impl RegisterBlock {
    #[doc = "0x00 - Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(&self) -> &Elcr {
        &self.elcr
    }
    #[doc = "0x02 - Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(&self, n: usize) -> &Elsegr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02 - Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub fn elsegr_iter(&self) -> impl Iterator<Item = &Elsegr> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x30 - Event Link Setting Register 8"]
    #[inline(always)]
    pub const fn elsr8(&self) -> &Elsr8 {
        &self.elsr8
    }
    #[doc = "0x48 - Event Link Setting Register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `ELSR14` register.</div>"]
    #[inline(always)]
    pub const fn elsr(&self, n: usize) -> &Elsr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(72)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x48 - Event Link Setting Register %s"]
    #[inline(always)]
    pub fn elsr_iter(&self) -> impl Iterator<Item = &Elsr> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(72)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x48 - Event Link Setting Register 14"]
    #[inline(always)]
    pub const fn elsr14(&self) -> &Elsr {
        self.elsr(0)
    }
    #[doc = "0x4c - Event Link Setting Register 15"]
    #[inline(always)]
    pub const fn elsr15(&self) -> &Elsr {
        self.elsr(1)
    }
    #[doc = "0x5c - Event Link Setting Register 19"]
    #[inline(always)]
    pub const fn elsr19(&self) -> &Elsr {
        &self.elsr19
    }
    #[doc = "0x5c - Event Link Setting Register 20"]
    #[inline(always)]
    pub const fn elsr20(&self) -> &Elsr {
        &self.elsr20
    }
    #[doc = "0x6c - Event Link Setting Register 23"]
    #[inline(always)]
    pub const fn elsr23(&self) -> &Elsr23 {
        &self.elsr23
    }
}
#[doc = "ELCR (rw) register accessor: Event Link Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`elcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elcr`]
module"]
#[doc(alias = "ELCR")]
pub type Elcr = crate::Reg<elcr::ElcrSpec>;
#[doc = "Event Link Controller Register"]
pub mod elcr;
#[doc = "ELSEGR (rw) register accessor: Event Link Software Event Generation Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsegr`]
module"]
#[doc(alias = "ELSEGR")]
pub type Elsegr = crate::Reg<elsegr::ElsegrSpec>;
#[doc = "Event Link Software Event Generation Register %s"]
pub mod elsegr;
pub use elsr as elsr8;
pub use Elsr as Elsr8;
#[doc = "ELSR (rw) register accessor: Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsr`]
module"]
#[doc(alias = "ELSR")]
pub type Elsr = crate::Reg<elsr::ElsrSpec>;
#[doc = "Event Link Setting Register %s"]
pub mod elsr;
pub use elsr as elsr19;
pub use elsr as elsr23;
pub use Elsr as Elsr19;
pub use Elsr as Elsr23;
