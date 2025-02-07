#[doc = "Register `ADCR3H` reader"]
pub type R = crate::R<Adcr3hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "8-bit A/D Conversion Result Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr3h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcr3hSpec;
impl crate::RegisterSpec for Adcr3hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcr3h::R`](R) reader structure"]
impl crate::Readable for Adcr3hSpec {}
#[doc = "`reset()` method sets ADCR3H to value 0"]
impl crate::Resettable for Adcr3hSpec {
    const RESET_VALUE: u8 = 0;
}
