#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    itlcmp0: [Itlcmp0; 2],
    itlcap00: Itlcap00,
    itlctl0: Itlctl0,
    itlcsel0: Itlcsel0,
    itlfdiv00: Itlfdiv00,
    itlfdiv01: Itlfdiv01,
    itlcc0: Itlcc0,
    itls0: Itls0,
    itlmkf0: Itlmkf0,
}
impl RegisterBlock {
    #[doc = "0x00 - Interval Timer Compare Registers 0%s"]
    #[inline(always)]
    pub const fn itlcmp0(&self, n: usize) -> &Itlcmp0 {
        &self.itlcmp0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Interval Timer Compare Registers 0%s"]
    #[inline(always)]
    pub fn itlcmp0_iter(&self) -> impl Iterator<Item = &Itlcmp0> {
        self.itlcmp0.iter()
    }
    #[doc = "0x04 - Interval Timer Capture Register 00"]
    #[inline(always)]
    pub const fn itlcap00(&self) -> &Itlcap00 {
        &self.itlcap00
    }
    #[doc = "0x06 - Interval Timer Control Register"]
    #[inline(always)]
    pub const fn itlctl0(&self) -> &Itlctl0 {
        &self.itlctl0
    }
    #[doc = "0x07 - Interval Timer Clock Select Register 0"]
    #[inline(always)]
    pub const fn itlcsel0(&self) -> &Itlcsel0 {
        &self.itlcsel0
    }
    #[doc = "0x08 - Interval Timer Frequency Division Register 0"]
    #[inline(always)]
    pub const fn itlfdiv00(&self) -> &Itlfdiv00 {
        &self.itlfdiv00
    }
    #[doc = "0x09 - Interval Timer Frequency Division Register 1"]
    #[inline(always)]
    pub const fn itlfdiv01(&self) -> &Itlfdiv01 {
        &self.itlfdiv01
    }
    #[doc = "0x0a - Interval Timer Capture Control Register 0"]
    #[inline(always)]
    pub const fn itlcc0(&self) -> &Itlcc0 {
        &self.itlcc0
    }
    #[doc = "0x0b - Interval Timer Status Register"]
    #[inline(always)]
    pub const fn itls0(&self) -> &Itls0 {
        &self.itls0
    }
    #[doc = "0x0c - Interval Timer Match Detection Mask Register"]
    #[inline(always)]
    pub const fn itlmkf0(&self) -> &Itlmkf0 {
        &self.itlmkf0
    }
}
#[doc = "ITLCMP0 (rw) register accessor: Interval Timer Compare Registers 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlcmp0`]
module"]
#[doc(alias = "ITLCMP0")]
pub type Itlcmp0 = crate::Reg<itlcmp0::Itlcmp0Spec>;
#[doc = "Interval Timer Compare Registers 0%s"]
pub mod itlcmp0;
#[doc = "ITLCAP00 (r) register accessor: Interval Timer Capture Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcap00::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlcap00`]
module"]
#[doc(alias = "ITLCAP00")]
pub type Itlcap00 = crate::Reg<itlcap00::Itlcap00Spec>;
#[doc = "Interval Timer Capture Register 00"]
pub mod itlcap00;
#[doc = "ITLCTL0 (rw) register accessor: Interval Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itlctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlctl0`]
module"]
#[doc(alias = "ITLCTL0")]
pub type Itlctl0 = crate::Reg<itlctl0::Itlctl0Spec>;
#[doc = "Interval Timer Control Register"]
pub mod itlctl0;
#[doc = "ITLCSEL0 (rw) register accessor: Interval Timer Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlcsel0`]
module"]
#[doc(alias = "ITLCSEL0")]
pub type Itlcsel0 = crate::Reg<itlcsel0::Itlcsel0Spec>;
#[doc = "Interval Timer Clock Select Register 0"]
pub mod itlcsel0;
#[doc = "ITLFDIV00 (rw) register accessor: Interval Timer Frequency Division Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlfdiv00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlfdiv00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlfdiv00`]
module"]
#[doc(alias = "ITLFDIV00")]
pub type Itlfdiv00 = crate::Reg<itlfdiv00::Itlfdiv00Spec>;
#[doc = "Interval Timer Frequency Division Register 0"]
pub mod itlfdiv00;
#[doc = "ITLFDIV01 (rw) register accessor: Interval Timer Frequency Division Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itlfdiv01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlfdiv01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlfdiv01`]
module"]
#[doc(alias = "ITLFDIV01")]
pub type Itlfdiv01 = crate::Reg<itlfdiv01::Itlfdiv01Spec>;
#[doc = "Interval Timer Frequency Division Register 1"]
pub mod itlfdiv01;
#[doc = "ITLCC0 (rw) register accessor: Interval Timer Capture Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlcc0`]
module"]
#[doc(alias = "ITLCC0")]
pub type Itlcc0 = crate::Reg<itlcc0::Itlcc0Spec>;
#[doc = "Interval Timer Capture Control Register 0"]
pub mod itlcc0;
#[doc = "ITLS0 (rw) register accessor: Interval Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itls0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itls0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itls0`]
module"]
#[doc(alias = "ITLS0")]
pub type Itls0 = crate::Reg<itls0::Itls0Spec>;
#[doc = "Interval Timer Status Register"]
pub mod itls0;
#[doc = "ITLMKF0 (rw) register accessor: Interval Timer Match Detection Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itlmkf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlmkf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itlmkf0`]
module"]
#[doc(alias = "ITLMKF0")]
pub type Itlmkf0 = crate::Reg<itlmkf0::Itlmkf0Spec>;
#[doc = "Interval Timer Match Detection Mask Register"]
pub mod itlmkf0;
