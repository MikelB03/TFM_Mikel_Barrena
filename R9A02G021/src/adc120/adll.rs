#[doc = "Register `ADLL` reader"]
pub type R = crate::R<AdllSpec>;
#[doc = "Register `ADLL` writer"]
pub type W = crate::W<AdllSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Conversion Result Comparison Lower Limit Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdllSpec;
impl crate::RegisterSpec for AdllSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adll::R`](R) reader structure"]
impl crate::Readable for AdllSpec {}
#[doc = "`write(|w| ..)` method takes [`adll::W`](W) writer structure"]
impl crate::Writable for AdllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADLL to value 0"]
impl crate::Resettable for AdllSpec {
    const RESET_VALUE: u8 = 0;
}
