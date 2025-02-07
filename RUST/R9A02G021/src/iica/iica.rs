#[doc = "Register `IICA%s` reader"]
pub type R = crate::R<IicaSpec>;
#[doc = "Register `IICA%s` writer"]
pub type W = crate::W<IicaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IICA Shift Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iica::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iica::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IicaSpec;
impl crate::RegisterSpec for IicaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iica::R`](R) reader structure"]
impl crate::Readable for IicaSpec {}
#[doc = "`write(|w| ..)` method takes [`iica::W`](W) writer structure"]
impl crate::Writable for IicaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICA%s to value 0"]
impl crate::Resettable for IicaSpec {
    const RESET_VALUE: u8 = 0;
}
