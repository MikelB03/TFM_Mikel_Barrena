#[doc = "Register `REMDAT%s` reader"]
pub type R = crate::R<RemdatSpec>;
#[doc = "Field `DAT` reader - Receive Data Store"]
pub type DatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Data Store"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new(self.bits)
    }
}
#[doc = "Receive Data %s Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remdat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemdatSpec;
impl crate::RegisterSpec for RemdatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remdat::R`](R) reader structure"]
impl crate::Readable for RemdatSpec {}
#[doc = "`reset()` method sets REMDAT%s to value 0"]
impl crate::Resettable for RemdatSpec {
    const RESET_VALUE: u8 = 0;
}
