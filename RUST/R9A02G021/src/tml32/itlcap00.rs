#[doc = "Register `ITLCAP00` reader"]
pub type R = crate::R<Itlcap00Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interval Timer Capture Register 00\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcap00::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlcap00Spec;
impl crate::RegisterSpec for Itlcap00Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`itlcap00::R`](R) reader structure"]
impl crate::Readable for Itlcap00Spec {}
#[doc = "`reset()` method sets ITLCAP00 to value 0"]
impl crate::Resettable for Itlcap00Spec {
    const RESET_VALUE: u16 = 0;
}
