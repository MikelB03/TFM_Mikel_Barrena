#[doc = "Register `TDR02` reader"]
pub type R = crate::R<Tdr02Spec>;
#[doc = "Register `TDR02` writer"]
pub type W = crate::W<Tdr02Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 02\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr02Spec;
impl crate::RegisterSpec for Tdr02Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr02::R`](R) reader structure"]
impl crate::Readable for Tdr02Spec {}
#[doc = "`write(|w| ..)` method takes [`tdr02::W`](W) writer structure"]
impl crate::Writable for Tdr02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TDR02 to value 0"]
impl crate::Resettable for Tdr02Spec {
    const RESET_VALUE: u16 = 0;
}
