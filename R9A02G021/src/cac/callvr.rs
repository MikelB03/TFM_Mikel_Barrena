#[doc = "Register `CALLVR` reader"]
pub type R = crate::R<CallvrSpec>;
#[doc = "Register `CALLVR` writer"]
pub type W = crate::W<CallvrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CAC Lower-Limit Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`callvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`callvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CallvrSpec;
impl crate::RegisterSpec for CallvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`callvr::R`](R) reader structure"]
impl crate::Readable for CallvrSpec {}
#[doc = "`write(|w| ..)` method takes [`callvr::W`](W) writer structure"]
impl crate::Writable for CallvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CALLVR to value 0"]
impl crate::Resettable for CallvrSpec {
    const RESET_VALUE: u16 = 0;
}
