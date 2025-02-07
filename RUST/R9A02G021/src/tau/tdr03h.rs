#[doc = "Register `TDR03H` reader"]
pub type R = crate::R<Tdr03hSpec>;
#[doc = "Register `TDR03H` writer"]
pub type W = crate::W<Tdr03hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer Data Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr03h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr03h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdr03hSpec;
impl crate::RegisterSpec for Tdr03hSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr03h::R`](R) reader structure"]
impl crate::Readable for Tdr03hSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr03h::W`](W) writer structure"]
impl crate::Writable for Tdr03hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TDR03H to value 0"]
impl crate::Resettable for Tdr03hSpec {
    const RESET_VALUE: u8 = 0;
}
