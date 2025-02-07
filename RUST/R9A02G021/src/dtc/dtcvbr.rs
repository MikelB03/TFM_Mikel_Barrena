#[doc = "Register `DTCVBR` reader"]
pub type R = crate::R<DtcvbrSpec>;
#[doc = "Register `DTCVBR` writer"]
pub type W = crate::W<DtcvbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DTC Vector Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcvbrSpec;
impl crate::RegisterSpec for DtcvbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcvbr::R`](R) reader structure"]
impl crate::Readable for DtcvbrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtcvbr::W`](W) writer structure"]
impl crate::Writable for DtcvbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTCVBR to value 0"]
impl crate::Resettable for DtcvbrSpec {
    const RESET_VALUE: u32 = 0;
}
