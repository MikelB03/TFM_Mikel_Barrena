#[doc = "Register `TDR01L` reader"]
pub type R = crate::R<Tdr01lSpec>;
#[doc = "Register `TDR01L` writer"]
pub type W = crate::W<Tdr01lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr01lSpec;
impl crate::RegisterSpec for Tdr01lSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr01l::R`](R) reader structure"]
impl crate::Readable for Tdr01lSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr01l::W`](W) writer structure"]
impl crate::Writable for Tdr01lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TDR01L to value 0"]
impl crate::Resettable for Tdr01lSpec {
    const RESET_VALUE: u8 = 0;
}
