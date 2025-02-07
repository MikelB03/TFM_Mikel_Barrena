#[doc = "Register `mtimecmp_lo` reader"]
pub type R = crate::R<MtimecmpLoSpec>;
#[doc = "Register `mtimecmp_lo` writer"]
pub type W = crate::W<MtimecmpLoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine Timer Comparator Register 0 Low\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmpLoSpec;
impl crate::RegisterSpec for MtimecmpLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_lo::R`](R) reader structure"]
impl crate::Readable for MtimecmpLoSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_lo::W`](W) writer structure"]
impl crate::Writable for MtimecmpLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtimecmp_lo to value 0xffff_ffff"]
impl crate::Resettable for MtimecmpLoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
