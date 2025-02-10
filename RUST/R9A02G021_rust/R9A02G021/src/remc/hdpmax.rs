#[doc = "Register `HDPMAX` reader"]
pub type R = crate::R<HdpmaxSpec>;
#[doc = "Register `HDPMAX` writer"]
pub type W = crate::W<HdpmaxSpec>;
#[doc = "Field `HDMAX` reader - Set the maximum width of the header pattern"]
pub type HdmaxR = crate::FieldReader<u16>;
#[doc = "Field `HDMAX` writer - Set the maximum width of the header pattern"]
pub type HdmaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Set the maximum width of the header pattern"]
    #[inline(always)]
    pub fn hdmax(&self) -> HdmaxR {
        HdmaxR::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the maximum width of the header pattern"]
    #[inline(always)]
    pub fn hdmax(&mut self) -> HdmaxW<HdpmaxSpec> {
        HdmaxW::new(self, 0)
    }
}
#[doc = "Header Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdpmaxSpec;
impl crate::RegisterSpec for HdpmaxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hdpmax::R`](R) reader structure"]
impl crate::Readable for HdpmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`hdpmax::W`](W) writer structure"]
impl crate::Writable for HdpmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HDPMAX to value 0"]
impl crate::Resettable for HdpmaxSpec {
    const RESET_VALUE: u16 = 0;
}
