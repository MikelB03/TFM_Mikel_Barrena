#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    compmdr: Compmdr,
    compfir: Compfir,
    compocr: Compocr,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Mode Setting Register"]
    #[inline(always)]
    pub const fn compmdr(&self) -> &Compmdr {
        &self.compmdr
    }
    #[doc = "0x01 - Comparator Filter Control Register"]
    #[inline(always)]
    pub const fn compfir(&self) -> &Compfir {
        &self.compfir
    }
    #[doc = "0x02 - Comparator Output Control Register"]
    #[inline(always)]
    pub const fn compocr(&self) -> &Compocr {
        &self.compocr
    }
}
#[doc = "COMPMDR (rw) register accessor: Comparator Mode Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compmdr`]
module"]
#[doc(alias = "COMPMDR")]
pub type Compmdr = crate::Reg<compmdr::CompmdrSpec>;
#[doc = "Comparator Mode Setting Register"]
pub mod compmdr;
#[doc = "COMPFIR (rw) register accessor: Comparator Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compfir`]
module"]
#[doc(alias = "COMPFIR")]
pub type Compfir = crate::Reg<compfir::CompfirSpec>;
#[doc = "Comparator Filter Control Register"]
pub mod compfir;
#[doc = "COMPOCR (rw) register accessor: Comparator Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compocr`]
module"]
#[doc(alias = "COMPOCR")]
pub type Compocr = crate::Reg<compocr::CompocrSpec>;
#[doc = "Comparator Output Control Register"]
pub mod compocr;
