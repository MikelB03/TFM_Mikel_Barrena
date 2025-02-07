#[doc = "Register `ADCRH` reader"]
pub type R = crate::R<AdcrhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "8-bit A/D Conversion Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcrhSpec;
impl crate::RegisterSpec for AdcrhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcrh::R`](R) reader structure"]
impl crate::Readable for AdcrhSpec {}
#[doc = "`reset()` method sets ADCRH to value 0"]
impl crate::Resettable for AdcrhSpec {
    const RESET_VALUE: u8 = 0;
}
