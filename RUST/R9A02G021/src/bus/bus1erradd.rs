#[doc = "Register `BUS1ERRADD` reader"]
pub type R = crate::R<Bus1erraddSpec>;
#[doc = "Field `BERAD` reader - Bus Error Address"]
pub type BeradR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error Address"]
    #[inline(always)]
    pub fn berad(&self) -> BeradR {
        BeradR::new(self.bits)
    }
}
#[doc = "Bus Error Address Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1erradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bus1erraddSpec;
impl crate::RegisterSpec for Bus1erraddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus1erradd::R`](R) reader structure"]
impl crate::Readable for Bus1erraddSpec {}
#[doc = "`reset()` method sets BUS1ERRADD to value 0"]
impl crate::Resettable for Bus1erraddSpec {
    const RESET_VALUE: u32 = 0;
}
