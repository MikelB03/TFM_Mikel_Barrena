#[doc = "Register `ADCR1H` reader"]
pub type R = crate::R<Adcr1hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "8-bit A/D Conversion Result Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr1h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr1hSpec;
impl crate::RegisterSpec for Adcr1hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcr1h::R`](R) reader structure"]
impl crate::Readable for Adcr1hSpec {}
#[doc = "`reset()` method sets ADCR1H to value 0"]
impl crate::Resettable for Adcr1hSpec {
    const RESET_VALUE: u8 = 0;
}
