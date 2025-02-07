#[doc = "Register `ADCR0H` reader"]
pub type R = crate::R<Adcr0hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "8-bit A/D Conversion Result Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr0h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr0hSpec;
impl crate::RegisterSpec for Adcr0hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcr0h::R`](R) reader structure"]
impl crate::Readable for Adcr0hSpec {}
#[doc = "`reset()` method sets ADCR0H to value 0"]
impl crate::Resettable for Adcr0hSpec {
    const RESET_VALUE: u8 = 0;
}
