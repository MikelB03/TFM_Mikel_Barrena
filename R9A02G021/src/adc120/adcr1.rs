#[doc = "Register `ADCR1` reader"]
pub type R = crate::R<Adcr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "12-bit or 10-bit A/D Conversion Result Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr1Spec;
impl crate::RegisterSpec for Adcr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcr1::R`](R) reader structure"]
impl crate::Readable for Adcr1Spec {}
#[doc = "`reset()` method sets ADCR1 to value 0"]
impl crate::Resettable for Adcr1Spec {
    const RESET_VALUE: u16 = 0;
}
