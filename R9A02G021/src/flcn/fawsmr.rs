#[doc = "Register `FAWSMR` reader"]
pub type R = crate::R<FawsmrSpec>;
#[doc = "Field `FAWS` reader - Access Window Start Address"]
pub type FawsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Access Window Start Address"]
    #[inline(always)]
    pub fn faws(&self) -> FawsR {
        FawsR::new(self.bits & 0x07ff)
    }
}
#[doc = "Flash Access Window Start Address Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fawsmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FawsmrSpec;
impl crate::RegisterSpec for FawsmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fawsmr::R`](R) reader structure"]
impl crate::Readable for FawsmrSpec {}
#[doc = "`reset()` method sets FAWSMR to value 0"]
impl crate::Resettable for FawsmrSpec {
    const RESET_VALUE: u16 = 0;
}
