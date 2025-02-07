#[doc = "Register `TDR04` reader"]
pub type R = crate::R<Tdr04Spec>;
#[doc = "Register `TDR04` writer"]
pub type W = crate::W<Tdr04Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 04\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr04Spec;
impl crate::RegisterSpec for Tdr04Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr04::R`](R) reader structure"]
impl crate::Readable for Tdr04Spec {}
#[doc = "`write(|w| ..)` method takes [`tdr04::W`](W) writer structure"]
impl crate::Writable for Tdr04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TDR04 to value 0"]
impl crate::Resettable for Tdr04Spec {
    const RESET_VALUE: u16 = 0;
}
