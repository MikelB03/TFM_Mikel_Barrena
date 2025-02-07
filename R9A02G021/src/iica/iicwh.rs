#[doc = "Register `IICWH%s` reader"]
pub type R = crate::R<IicwhSpec>;
#[doc = "Register `IICWH%s` writer"]
pub type W = crate::W<IicwhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IICA High-level Width Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicwh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicwh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IicwhSpec;
impl crate::RegisterSpec for IicwhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iicwh::R`](R) reader structure"]
impl crate::Readable for IicwhSpec {}
#[doc = "`write(|w| ..)` method takes [`iicwh::W`](W) writer structure"]
impl crate::Writable for IicwhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICWH%s to value 0xff"]
impl crate::Resettable for IicwhSpec {
    const RESET_VALUE: u8 = 0xff;
}
