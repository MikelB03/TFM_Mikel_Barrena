#[doc = "Register `IICWL%s` reader"]
pub type R = crate::R<IicwlSpec>;
#[doc = "Register `IICWL%s` writer"]
pub type W = crate::W<IicwlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IICA Low-level Width Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicwl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicwl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IicwlSpec;
impl crate::RegisterSpec for IicwlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iicwl::R`](R) reader structure"]
impl crate::Readable for IicwlSpec {}
#[doc = "`write(|w| ..)` method takes [`iicwl::W`](W) writer structure"]
impl crate::Writable for IicwlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICWL%s to value 0xff"]
impl crate::Resettable for IicwlSpec {
    const RESET_VALUE: u8 = 0xff;
}
