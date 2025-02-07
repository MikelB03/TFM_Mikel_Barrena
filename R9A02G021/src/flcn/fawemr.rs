#[doc = "Register `FAWEMR` reader"]
pub type R = crate::R<FawemrSpec>;
#[doc = "Field `FAWE` reader - Access Window End Address"]
pub type FaweR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Access Window End Address"]
    #[inline(always)]
    pub fn fawe(&self) -> FaweR {
        FaweR::new(self.bits & 0x07ff)
    }
}
#[doc = "Flash Access Window End Address Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fawemr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FawemrSpec;
impl crate::RegisterSpec for FawemrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fawemr::R`](R) reader structure"]
impl crate::Readable for FawemrSpec {}
#[doc = "`reset()` method sets FAWEMR to value 0"]
impl crate::Resettable for FawemrSpec {
    const RESET_VALUE: u16 = 0;
}
