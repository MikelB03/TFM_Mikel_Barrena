#[doc = "Register `ADCR2` reader"]
pub type R = crate::R<Adcr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "12-bit or 10-bit A/D Conversion Result Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr2Spec;
impl crate::RegisterSpec for Adcr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcr2::R`](R) reader structure"]
impl crate::Readable for Adcr2Spec {}
#[doc = "`reset()` method sets ADCR2 to value 0"]
impl crate::Resettable for Adcr2Spec {
    const RESET_VALUE: u16 = 0;
}
