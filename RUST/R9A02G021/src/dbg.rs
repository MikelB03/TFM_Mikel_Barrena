#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    progbuf: [Progbuf; 8],
    _reserved1: [u8; 0x20],
    data: [Data; 4],
}
impl RegisterBlock {
    #[doc = "0x80..0xa0 - Program Buffer 0 to 7"]
    #[inline(always)]
    pub const fn progbuf(&self, n: usize) -> &Progbuf {
        &self.progbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xa0 - Program Buffer 0 to 7"]
    #[inline(always)]
    pub fn progbuf_iter(&self) -> impl Iterator<Item = &Progbuf> {
        self.progbuf.iter()
    }
    #[doc = "0xc0..0xd0 - Abstract Data 0 to 3"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xd0 - Abstract Data 0 to 3"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
}
#[doc = "progbuf (rw) register accessor: Program Buffer 0 to 7\n\nYou can [`read`](crate::Reg::read) this register and get [`progbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progbuf`]
module"]
#[doc(alias = "progbuf")]
pub type Progbuf = crate::Reg<progbuf::ProgbufSpec>;
#[doc = "Program Buffer 0 to 7"]
pub mod progbuf;
#[doc = "data (rw) register accessor: Abstract Data 0 to 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Abstract Data 0 to 3"]
pub mod data;
