#[doc = "Register `D1PMAX` reader"]
pub type R = crate::R<D1pmaxSpec>;
#[doc = "Register `D1PMAX` writer"]
pub type W = crate::W<D1pmaxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data 1 Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1pmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1pmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1pmaxSpec;
impl crate::RegisterSpec for D1pmaxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d1pmax::R`](R) reader structure"]
impl crate::Readable for D1pmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`d1pmax::W`](W) writer structure"]
impl crate::Writable for D1pmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets D1PMAX to value 0"]
impl crate::Resettable for D1pmaxSpec {
    const RESET_VALUE: u8 = 0;
}
