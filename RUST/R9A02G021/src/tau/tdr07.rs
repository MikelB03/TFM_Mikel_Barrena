#[doc = "Register `TDR07` reader"]
pub type R = crate::R<Tdr07Spec>;
#[doc = "Register `TDR07` writer"]
pub type W = crate::W<Tdr07Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 07\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr07Spec;
impl crate::RegisterSpec for Tdr07Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr07::R`](R) reader structure"]
impl crate::Readable for Tdr07Spec {}
#[doc = "`write(|w| ..)` method takes [`tdr07::W`](W) writer structure"]
impl crate::Writable for Tdr07Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TDR07 to value 0"]
impl crate::Resettable for Tdr07Spec {
    const RESET_VALUE: u16 = 0;
}
