#[doc = "Register `IWDTRR` reader"]
pub type R = crate::R<IwdtrrSpec>;
#[doc = "Register `IWDTRR` writer"]
pub type W = crate::W<IwdtrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IWDT Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdtrrSpec;
impl crate::RegisterSpec for IwdtrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iwdtrr::R`](R) reader structure"]
impl crate::Readable for IwdtrrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdtrr::W`](W) writer structure"]
impl crate::Writable for IwdtrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IWDTRR to value 0xff"]
impl crate::Resettable for IwdtrrSpec {
    const RESET_VALUE: u8 = 0xff;
}
