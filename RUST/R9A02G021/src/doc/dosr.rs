#[doc = "Register `DOSR` reader"]
pub type R = crate::R<DosrSpec>;
#[doc = "Field `DOPCF` reader - Data Operation Circuit Flag"]
pub type DopcfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Operation Circuit Flag"]
    #[inline(always)]
    pub fn dopcf(&self) -> DopcfR {
        DopcfR::new((self.bits & 1) != 0)
    }
}
#[doc = "DOC Flag Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dosr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DosrSpec;
impl crate::RegisterSpec for DosrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dosr::R`](R) reader structure"]
impl crate::Readable for DosrSpec {}
#[doc = "`reset()` method sets DOSR to value 0"]
impl crate::Resettable for DosrSpec {
    const RESET_VALUE: u8 = 0;
}
