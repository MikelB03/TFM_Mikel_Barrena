#[doc = "Register `REMTIM` reader"]
pub type R = crate::R<RemtimSpec>;
#[doc = "Field `TIM` reader - Measurement Result"]
pub type TimR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Measurement Result"]
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(self.bits & 0x07ff)
    }
}
#[doc = "Measurement Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remtim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemtimSpec;
impl crate::RegisterSpec for RemtimSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`remtim::R`](R) reader structure"]
impl crate::Readable for RemtimSpec {}
#[doc = "`reset()` method sets REMTIM to value 0"]
impl crate::Resettable for RemtimSpec {
    const RESET_VALUE: u16 = 0;
}
