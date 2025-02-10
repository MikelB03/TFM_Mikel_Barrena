#[doc = "Register `CAULVR` reader"]
pub type R = crate::R<CaulvrSpec>;
#[doc = "Register `CAULVR` writer"]
pub type W = crate::W<CaulvrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CAC Upper-Limit Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`caulvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaulvrSpec;
impl crate::RegisterSpec for CaulvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`caulvr::R`](R) reader structure"]
impl crate::Readable for CaulvrSpec {}
#[doc = "`write(|w| ..)` method takes [`caulvr::W`](W) writer structure"]
impl crate::Writable for CaulvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CAULVR to value 0"]
impl crate::Resettable for CaulvrSpec {
    const RESET_VALUE: u16 = 0;
}
