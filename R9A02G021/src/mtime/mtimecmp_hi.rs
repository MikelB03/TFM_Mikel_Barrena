#[doc = "Register `mtimecmp_hi` reader"]
pub type R = crate::R<MtimecmpHiSpec>;
#[doc = "Register `mtimecmp_hi` writer"]
pub type W = crate::W<MtimecmpHiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine Timer Comparator Register 0 High\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmpHiSpec;
impl crate::RegisterSpec for MtimecmpHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_hi::R`](R) reader structure"]
impl crate::Readable for MtimecmpHiSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_hi::W`](W) writer structure"]
impl crate::Writable for MtimecmpHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtimecmp_hi to value 0xffff_ffff"]
impl crate::Resettable for MtimecmpHiSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
