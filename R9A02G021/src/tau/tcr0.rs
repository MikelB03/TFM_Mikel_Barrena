#[doc = "Register `TCR0%s` reader"]
pub type R = crate::R<Tcr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Timer Counter Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr0Spec;
impl crate::RegisterSpec for Tcr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcr0::R`](R) reader structure"]
impl crate::Readable for Tcr0Spec {}
#[doc = "`reset()` method sets TCR0%s to value 0xffff"]
impl crate::Resettable for Tcr0Spec {
    const RESET_VALUE: u16 = 0xffff;
}
