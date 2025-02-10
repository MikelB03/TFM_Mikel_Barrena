#[doc = "Register `TDR01H` reader"]
pub type R = crate::R<Tdr01hSpec>;
#[doc = "Register `TDR01H` writer"]
pub type W = crate::W<Tdr01hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr01h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr01h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr01hSpec;
impl crate::RegisterSpec for Tdr01hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr01h::R`](R) reader structure"]
impl crate::Readable for Tdr01hSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr01h::W`](W) writer structure"]
impl crate::Writable for Tdr01hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TDR01H to value 0"]
impl crate::Resettable for Tdr01hSpec {
    const RESET_VALUE: u8 = 0;
}
