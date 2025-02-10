#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adm0: Adm0,
    ads: Ads,
    adm1: Adm1,
    _reserved3: [u8; 0x03],
    _reserved_3_adcr: [u8; 0x02],
    _reserved4: [u8; 0x88],
    adm2: Adm2,
    adul: Adul,
    adll: Adll,
    adtes: Adtes,
    _reserved8: [u8; 0x0c],
    _reserved_8_adcr: [u8; 0x02],
    _reserved_9_adcr: [u8; 0x02],
    _reserved_10_adcr: [u8; 0x02],
    _reserved_11_adcr: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - A/D Converter Mode Register 0"]
    #[inline(always)]
    pub const fn adm0(&self) -> &Adm0 {
        &self.adm0
    }
    #[doc = "0x01 - Analog Input Channel Specification Register"]
    #[inline(always)]
    pub const fn ads(&self) -> &Ads {
        &self.ads
    }
    #[doc = "0x02 - A/D Converter Mode Register 1"]
    #[inline(always)]
    pub const fn adm1(&self) -> &Adm1 {
        &self.adm1
    }
    #[doc = "0x06 - 12-bit or 10-bit A/D Conversion Result Register"]
    #[inline(always)]
    pub const fn adcr(&self) -> &Adcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - 8-bit A/D Conversion Result Register"]
    #[inline(always)]
    pub const fn adcrh(&self) -> &Adcrh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x90 - A/D Converter Mode Register 2"]
    #[inline(always)]
    pub const fn adm2(&self) -> &Adm2 {
        &self.adm2
    }
    #[doc = "0x91 - Conversion Result Comparison Upper Limit Setting Register"]
    #[inline(always)]
    pub const fn adul(&self) -> &Adul {
        &self.adul
    }
    #[doc = "0x92 - Conversion Result Comparison Lower Limit Setting Register"]
    #[inline(always)]
    pub const fn adll(&self) -> &Adll {
        &self.adll
    }
    #[doc = "0x93 - A/D Test Register"]
    #[inline(always)]
    pub const fn adtes(&self) -> &Adtes {
        &self.adtes
    }
    #[doc = "0xa0 - 12-bit or 10-bit A/D Conversion Result Register 0"]
    #[inline(always)]
    pub const fn adcr0(&self) -> &Adcr0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa1 - 8-bit A/D Conversion Result Register 0"]
    #[inline(always)]
    pub const fn adcr0h(&self) -> &Adcr0h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(161).cast() }
    }
    #[doc = "0xa2 - 12-bit or 10-bit A/D Conversion Result Register 1"]
    #[inline(always)]
    pub const fn adcr1(&self) -> &Adcr1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(162).cast() }
    }
    #[doc = "0xa3 - 8-bit A/D Conversion Result Register 1"]
    #[inline(always)]
    pub const fn adcr1h(&self) -> &Adcr1h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(163).cast() }
    }
    #[doc = "0xa4 - 12-bit or 10-bit A/D Conversion Result Register 2"]
    #[inline(always)]
    pub const fn adcr2(&self) -> &Adcr2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa5 - 8-bit A/D Conversion Result Register 2"]
    #[inline(always)]
    pub const fn adcr2h(&self) -> &Adcr2h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(165).cast() }
    }
    #[doc = "0xa6 - 12-bit or 10-bit A/D Conversion Result Register 3"]
    #[inline(always)]
    pub const fn adcr3(&self) -> &Adcr3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(166).cast() }
    }
    #[doc = "0xa7 - 8-bit A/D Conversion Result Register 3"]
    #[inline(always)]
    pub const fn adcr3h(&self) -> &Adcr3h {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(167).cast() }
    }
}
#[doc = "ADM0 (rw) register accessor: A/D Converter Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adm0`]
module"]
#[doc(alias = "ADM0")]
pub type Adm0 = crate::Reg<adm0::Adm0Spec>;
#[doc = "A/D Converter Mode Register 0"]
pub mod adm0;
#[doc = "ADS (rw) register accessor: Analog Input Channel Specification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ads::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ads::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ads`]
module"]
#[doc(alias = "ADS")]
pub type Ads = crate::Reg<ads::AdsSpec>;
#[doc = "Analog Input Channel Specification Register"]
pub mod ads;
#[doc = "ADM1 (rw) register accessor: A/D Converter Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adm1`]
module"]
#[doc(alias = "ADM1")]
pub type Adm1 = crate::Reg<adm1::Adm1Spec>;
#[doc = "A/D Converter Mode Register 1"]
pub mod adm1;
#[doc = "ADCR (r) register accessor: 12-bit or 10-bit A/D Conversion Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr`]
module"]
#[doc(alias = "ADCR")]
pub type Adcr = crate::Reg<adcr::AdcrSpec>;
#[doc = "12-bit or 10-bit A/D Conversion Result Register"]
pub mod adcr;
#[doc = "ADCRH (r) register accessor: 8-bit A/D Conversion Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcrh`]
module"]
#[doc(alias = "ADCRH")]
pub type Adcrh = crate::Reg<adcrh::AdcrhSpec>;
#[doc = "8-bit A/D Conversion Result Register"]
pub mod adcrh;
#[doc = "ADM2 (rw) register accessor: A/D Converter Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adm2`]
module"]
#[doc(alias = "ADM2")]
pub type Adm2 = crate::Reg<adm2::Adm2Spec>;
#[doc = "A/D Converter Mode Register 2"]
pub mod adm2;
#[doc = "ADUL (rw) register accessor: Conversion Result Comparison Upper Limit Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adul`]
module"]
#[doc(alias = "ADUL")]
pub type Adul = crate::Reg<adul::AdulSpec>;
#[doc = "Conversion Result Comparison Upper Limit Setting Register"]
pub mod adul;
#[doc = "ADLL (rw) register accessor: Conversion Result Comparison Lower Limit Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adll`]
module"]
#[doc(alias = "ADLL")]
pub type Adll = crate::Reg<adll::AdllSpec>;
#[doc = "Conversion Result Comparison Lower Limit Setting Register"]
pub mod adll;
#[doc = "ADTES (rw) register accessor: A/D Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adtes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adtes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adtes`]
module"]
#[doc(alias = "ADTES")]
pub type Adtes = crate::Reg<adtes::AdtesSpec>;
#[doc = "A/D Test Register"]
pub mod adtes;
#[doc = "ADCR0 (r) register accessor: 12-bit or 10-bit A/D Conversion Result Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr0`]
module"]
#[doc(alias = "ADCR0")]
pub type Adcr0 = crate::Reg<adcr0::Adcr0Spec>;
#[doc = "12-bit or 10-bit A/D Conversion Result Register 0"]
pub mod adcr0;
#[doc = "ADCR0H (r) register accessor: 8-bit A/D Conversion Result Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr0h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr0h`]
module"]
#[doc(alias = "ADCR0H")]
pub type Adcr0h = crate::Reg<adcr0h::Adcr0hSpec>;
#[doc = "8-bit A/D Conversion Result Register 0"]
pub mod adcr0h;
#[doc = "ADCR1 (r) register accessor: 12-bit or 10-bit A/D Conversion Result Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr1`]
module"]
#[doc(alias = "ADCR1")]
pub type Adcr1 = crate::Reg<adcr1::Adcr1Spec>;
#[doc = "12-bit or 10-bit A/D Conversion Result Register 1"]
pub mod adcr1;
#[doc = "ADCR1H (r) register accessor: 8-bit A/D Conversion Result Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr1h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr1h`]
module"]
#[doc(alias = "ADCR1H")]
pub type Adcr1h = crate::Reg<adcr1h::Adcr1hSpec>;
#[doc = "8-bit A/D Conversion Result Register 1"]
pub mod adcr1h;
#[doc = "ADCR2 (r) register accessor: 12-bit or 10-bit A/D Conversion Result Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr2`]
module"]
#[doc(alias = "ADCR2")]
pub type Adcr2 = crate::Reg<adcr2::Adcr2Spec>;
#[doc = "12-bit or 10-bit A/D Conversion Result Register 2"]
pub mod adcr2;
#[doc = "ADCR2H (r) register accessor: 8-bit A/D Conversion Result Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr2h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr2h`]
module"]
#[doc(alias = "ADCR2H")]
pub type Adcr2h = crate::Reg<adcr2h::Adcr2hSpec>;
#[doc = "8-bit A/D Conversion Result Register 2"]
pub mod adcr2h;
#[doc = "ADCR3 (r) register accessor: 12-bit or 10-bit A/D Conversion Result Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr3`]
module"]
#[doc(alias = "ADCR3")]
pub type Adcr3 = crate::Reg<adcr3::Adcr3Spec>;
#[doc = "12-bit or 10-bit A/D Conversion Result Register 3"]
pub mod adcr3;
#[doc = "ADCR3H (r) register accessor: 8-bit A/D Conversion Result Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr3h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr3h`]
module"]
#[doc(alias = "ADCR3H")]
pub type Adcr3h = crate::Reg<adcr3h::Adcr3hSpec>;
#[doc = "8-bit A/D Conversion Result Register 3"]
pub mod adcr3h;
