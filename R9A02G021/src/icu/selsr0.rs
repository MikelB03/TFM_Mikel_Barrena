#[doc = "Register `SELSR0` reader"]
pub type R = crate::R<Selsr0Spec>;
#[doc = "Register `SELSR0` writer"]
pub type W = crate::W<Selsr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SYS Event Link Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Selsr0Spec;
impl crate::RegisterSpec for Selsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`selsr0::R`](R) reader structure"]
impl crate::Readable for Selsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`selsr0::W`](W) writer structure"]
impl crate::Writable for Selsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SELSR0 to value 0"]
impl crate::Resettable for Selsr0Spec {
    const RESET_VALUE: u16 = 0;
}
