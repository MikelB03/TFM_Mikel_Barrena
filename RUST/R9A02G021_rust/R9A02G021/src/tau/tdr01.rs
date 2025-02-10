#[doc = "Register `TDR01` reader"]
pub type R = crate::R<Tdr01Spec>;
#[doc = "Register `TDR01` writer"]
pub type W = crate::W<Tdr01Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr01Spec;
impl crate::RegisterSpec for Tdr01Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr01::R`](R) reader structure"]
impl crate::Readable for Tdr01Spec {}
#[doc = "`write(|w| ..)` method takes [`tdr01::W`](W) writer structure"]
impl crate::Writable for Tdr01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TDR01 to value 0"]
impl crate::Resettable for Tdr01Spec {
    const RESET_VALUE: u16 = 0;
}
