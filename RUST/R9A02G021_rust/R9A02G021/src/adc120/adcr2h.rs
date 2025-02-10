#[doc = "Register `ADCR2H` reader"]
pub type R = crate::R<Adcr2hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "8-bit A/D Conversion Result Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr2h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr2hSpec;
impl crate::RegisterSpec for Adcr2hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcr2h::R`](R) reader structure"]
impl crate::Readable for Adcr2hSpec {}
#[doc = "`reset()` method sets ADCR2H to value 0"]
impl crate::Resettable for Adcr2hSpec {
    const RESET_VALUE: u8 = 0;
}
