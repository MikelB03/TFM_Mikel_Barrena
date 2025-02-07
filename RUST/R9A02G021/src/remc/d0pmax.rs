#[doc = "Register `D0PMAX` reader"]
pub type R = crate::R<D0pmaxSpec>;
#[doc = "Register `D0PMAX` writer"]
pub type W = crate::W<D0pmaxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data 0 Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0pmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0pmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0pmaxSpec;
impl crate::RegisterSpec for D0pmaxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d0pmax::R`](R) reader structure"]
impl crate::Readable for D0pmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`d0pmax::W`](W) writer structure"]
impl crate::Writable for D0pmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets D0PMAX to value 0"]
impl crate::Resettable for D0pmaxSpec {
    const RESET_VALUE: u8 = 0;
}
