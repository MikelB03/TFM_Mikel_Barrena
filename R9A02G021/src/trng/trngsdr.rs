#[doc = "Register `TRNGSDR` reader"]
pub type R = crate::R<TrngsdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Random Number Seed Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trngsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngsdrSpec;
impl crate::RegisterSpec for TrngsdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trngsdr::R`](R) reader structure"]
impl crate::Readable for TrngsdrSpec {}
#[doc = "`reset()` method sets TRNGSDR to value 0"]
impl crate::Resettable for TrngsdrSpec {
    const RESET_VALUE: u8 = 0;
}
