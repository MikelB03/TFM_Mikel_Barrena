#[doc = "Register `D1PMIN` reader"]
pub type R = crate::R<D1pminSpec>;
#[doc = "Register `D1PMIN` writer"]
pub type W = crate::W<D1pminSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data 1 Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1pmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1pmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1pminSpec;
impl crate::RegisterSpec for D1pminSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d1pmin::R`](R) reader structure"]
impl crate::Readable for D1pminSpec {}
#[doc = "`write(|w| ..)` method takes [`d1pmin::W`](W) writer structure"]
impl crate::Writable for D1pminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets D1PMIN to value 0"]
impl crate::Resettable for D1pminSpec {
    const RESET_VALUE: u8 = 0;
}
