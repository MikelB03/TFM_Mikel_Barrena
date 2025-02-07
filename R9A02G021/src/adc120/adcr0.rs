#[doc = "Register `ADCR0` reader"]
pub type R = crate::R<Adcr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "12-bit or 10-bit A/D Conversion Result Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr0Spec;
impl crate::RegisterSpec for Adcr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcr0::R`](R) reader structure"]
impl crate::Readable for Adcr0Spec {}
#[doc = "`reset()` method sets ADCR0 to value 0"]
impl crate::Resettable for Adcr0Spec {
    const RESET_VALUE: u16 = 0;
}
