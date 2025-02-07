#[doc = "Register `HOCOUTCR` reader"]
pub type R = crate::R<HocoutcrSpec>;
#[doc = "Register `HOCOUTCR` writer"]
pub type W = crate::W<HocoutcrSpec>;
#[doc = "Field `HOCOUTRM` reader - HOCO User Trimming"]
pub type HocoutrmR = crate::FieldReader;
#[doc = "Field `HOCOUTRM` writer - HOCO User Trimming"]
pub type HocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HOCO User Trimming"]
    #[inline(always)]
    pub fn hocoutrm(&self) -> HocoutrmR {
        HocoutrmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HOCO User Trimming"]
    #[inline(always)]
    pub fn hocoutrm(&mut self) -> HocoutrmW<HocoutcrSpec> {
        HocoutrmW::new(self, 0)
    }
}
#[doc = "HOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HocoutcrSpec;
impl crate::RegisterSpec for HocoutcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hocoutcr::R`](R) reader structure"]
impl crate::Readable for HocoutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hocoutcr::W`](W) writer structure"]
impl crate::Writable for HocoutcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOCOUTCR to value 0"]
impl crate::Resettable for HocoutcrSpec {
    const RESET_VALUE: u8 = 0;
}
