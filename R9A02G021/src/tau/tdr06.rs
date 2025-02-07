#[doc = "Register `TDR06` reader"]
pub type R = crate::R<Tdr06Spec>;
#[doc = "Register `TDR06` writer"]
pub type W = crate::W<Tdr06Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 06\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr06Spec;
impl crate::RegisterSpec for Tdr06Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr06::R`](R) reader structure"]
impl crate::Readable for Tdr06Spec {}
#[doc = "`write(|w| ..)` method takes [`tdr06::W`](W) writer structure"]
impl crate::Writable for Tdr06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TDR06 to value 0"]
impl crate::Resettable for Tdr06Spec {
    const RESET_VALUE: u16 = 0;
}
