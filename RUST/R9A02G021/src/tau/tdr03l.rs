#[doc = "Register `TDR03L` reader"]
pub type R = crate::R<Tdr03lSpec>;
#[doc = "Register `TDR03L` writer"]
pub type W = crate::W<Tdr03lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr03l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr03l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr03lSpec;
impl crate::RegisterSpec for Tdr03lSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr03l::R`](R) reader structure"]
impl crate::Readable for Tdr03lSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr03l::W`](W) writer structure"]
impl crate::Writable for Tdr03lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TDR03L to value 0"]
impl crate::Resettable for Tdr03lSpec {
    const RESET_VALUE: u8 = 0;
}
