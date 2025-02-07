#[doc = "Register `D0PMIN` reader"]
pub type R = crate::R<D0pminSpec>;
#[doc = "Register `D0PMIN` writer"]
pub type W = crate::W<D0pminSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data 0 Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0pmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0pmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0pminSpec;
impl crate::RegisterSpec for D0pminSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d0pmin::R`](R) reader structure"]
impl crate::Readable for D0pminSpec {}
#[doc = "`write(|w| ..)` method takes [`d0pmin::W`](W) writer structure"]
impl crate::Writable for D0pminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets D0PMIN to value 0"]
impl crate::Resettable for D0pminSpec {
    const RESET_VALUE: u8 = 0;
}
