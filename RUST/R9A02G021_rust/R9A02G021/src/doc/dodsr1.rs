#[doc = "Register `DODSR1` reader"]
pub type R = crate::R<Dodsr1Spec>;
#[doc = "Register `DODSR1` writer"]
pub type W = crate::W<Dodsr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DOC Data Setting Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dodsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dodsr1Spec;
impl crate::RegisterSpec for Dodsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dodsr1::R`](R) reader structure"]
impl crate::Readable for Dodsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dodsr1::W`](W) writer structure"]
impl crate::Writable for Dodsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DODSR1 to value 0"]
impl crate::Resettable for Dodsr1Spec {
    const RESET_VALUE: u32 = 0;
}
