#[doc = "Register `RXBA%s` reader"]
pub type R = crate::R<RxbaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Receive Buffer Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rxba::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbaSpec;
impl crate::RegisterSpec for RxbaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxba::R`](R) reader structure"]
impl crate::Readable for RxbaSpec {}
#[doc = "`reset()` method sets RXBA%s to value 0xff"]
impl crate::Resettable for RxbaSpec {
    const RESET_VALUE: u8 = 0xff;
}
