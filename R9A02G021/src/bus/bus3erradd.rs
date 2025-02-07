#[doc = "Register `BUS3ERRADD` reader"]
pub type R = crate::R<Bus3erraddSpec>;
#[doc = "Field `BERAD` reader - Bus Error Address"]
pub type BeradR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error Address"]
    #[inline(always)]
    pub fn berad(&self) -> BeradR {
        BeradR::new(self.bits)
    }
}
#[doc = "Bus Error Address Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bus3erradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bus3erraddSpec;
impl crate::RegisterSpec for Bus3erraddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus3erradd::R`](R) reader structure"]
impl crate::Readable for Bus3erraddSpec {}
#[doc = "`reset()` method sets BUS3ERRADD to value 0"]
impl crate::Resettable for Bus3erraddSpec {
    const RESET_VALUE: u32 = 0;
}
