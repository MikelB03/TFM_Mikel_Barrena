#[doc = "Register `ADCR` reader"]
pub type R = crate::R<AdcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "12-bit or 10-bit A/D Conversion Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcrSpec;
impl crate::RegisterSpec for AdcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcr::R`](R) reader structure"]
impl crate::Readable for AdcrSpec {}
#[doc = "`reset()` method sets ADCR to value 0"]
impl crate::Resettable for AdcrSpec {
    const RESET_VALUE: u16 = 0;
}
