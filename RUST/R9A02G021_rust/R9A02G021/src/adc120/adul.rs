#[doc = "Register `ADUL` reader"]
pub type R = crate::R<AdulSpec>;
#[doc = "Register `ADUL` writer"]
pub type W = crate::W<AdulSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Conversion Result Comparison Upper Limit Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdulSpec;
impl crate::RegisterSpec for AdulSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adul::R`](R) reader structure"]
impl crate::Readable for AdulSpec {}
#[doc = "`write(|w| ..)` method takes [`adul::W`](W) writer structure"]
impl crate::Writable for AdulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADUL to value 0xff"]
impl crate::Resettable for AdulSpec {
    const RESET_VALUE: u8 = 0xff;
}
