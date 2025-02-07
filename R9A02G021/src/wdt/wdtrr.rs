#[doc = "Register `WDTRR` reader"]
pub type R = crate::R<WdtrrSpec>;
#[doc = "Register `WDTRR` writer"]
pub type W = crate::W<WdtrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "WDT Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtrrSpec;
impl crate::RegisterSpec for WdtrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtrr::R`](R) reader structure"]
impl crate::Readable for WdtrrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtrr::W`](W) writer structure"]
impl crate::Writable for WdtrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDTRR to value 0xff"]
impl crate::Resettable for WdtrrSpec {
    const RESET_VALUE: u8 = 0xff;
}
