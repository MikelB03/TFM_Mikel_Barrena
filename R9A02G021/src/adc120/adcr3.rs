#[doc = "Register `ADCR3` reader"]
pub type R = crate::R<Adcr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "12-bit or 10-bit A/D Conversion Result Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr3Spec;
impl crate::RegisterSpec for Adcr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcr3::R`](R) reader structure"]
impl crate::Readable for Adcr3Spec {}
#[doc = "`reset()` method sets ADCR3 to value 0"]
impl crate::Resettable for Adcr3Spec {
    const RESET_VALUE: u16 = 0;
}
