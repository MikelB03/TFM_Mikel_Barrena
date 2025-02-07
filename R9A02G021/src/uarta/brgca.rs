#[doc = "Register `BRGCA%s` reader"]
pub type R = crate::R<BrgcaSpec>;
#[doc = "Register `BRGCA%s` writer"]
pub type W = crate::W<BrgcaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Baud Rate Generator Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`brgca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brgca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgcaSpec;
impl crate::RegisterSpec for BrgcaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`brgca::R`](R) reader structure"]
impl crate::Readable for BrgcaSpec {}
#[doc = "`write(|w| ..)` method takes [`brgca::W`](W) writer structure"]
impl crate::Writable for BrgcaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BRGCA%s to value 0xff"]
impl crate::Resettable for BrgcaSpec {
    const RESET_VALUE: u8 = 0xff;
}
