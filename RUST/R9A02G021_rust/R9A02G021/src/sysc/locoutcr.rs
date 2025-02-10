#[doc = "Register `LOCOUTCR` reader"]
pub type R = crate::R<LocoutcrSpec>;
#[doc = "Register `LOCOUTCR` writer"]
pub type W = crate::W<LocoutcrSpec>;
#[doc = "Field `LOCOUTRM` reader - LOCO User Trimming"]
pub type LocoutrmR = crate::FieldReader;
#[doc = "Field `LOCOUTRM` writer - LOCO User Trimming"]
pub type LocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LOCO User Trimming"]
    #[inline(always)]
    pub fn locoutrm(&self) -> LocoutrmR {
        LocoutrmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCO User Trimming"]
    #[inline(always)]
    pub fn locoutrm(&mut self) -> LocoutrmW<LocoutcrSpec> {
        LocoutrmW::new(self, 0)
    }
}
#[doc = "LOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`locoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LocoutcrSpec;
impl crate::RegisterSpec for LocoutcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`locoutcr::R`](R) reader structure"]
impl crate::Readable for LocoutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`locoutcr::W`](W) writer structure"]
impl crate::Writable for LocoutcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LOCOUTCR to value 0"]
impl crate::Resettable for LocoutcrSpec {
    const RESET_VALUE: u8 = 0;
}
