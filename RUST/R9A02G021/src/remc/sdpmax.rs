#[doc = "Register `SDPMAX` reader"]
pub type R = crate::R<SdpmaxSpec>;
#[doc = "Register `SDPMAX` writer"]
pub type W = crate::W<SdpmaxSpec>;
#[doc = "Field `SDMAX` reader - Set the maximum width of the special data pattern"]
pub type SdmaxR = crate::FieldReader<u16>;
#[doc = "Field `SDMAX` writer - Set the maximum width of the special data pattern"]
pub type SdmaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Set the maximum width of the special data pattern"]
    #[inline(always)]
    pub fn sdmax(&self) -> SdmaxR {
        SdmaxR::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the maximum width of the special data pattern"]
    #[inline(always)]
    pub fn sdmax(&mut self) -> SdmaxW<SdpmaxSpec> {
        SdmaxW::new(self, 0)
    }
}
#[doc = "Special Data Pattern Maximum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdpmaxSpec;
impl crate::RegisterSpec for SdpmaxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdpmax::R`](R) reader structure"]
impl crate::Readable for SdpmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`sdpmax::W`](W) writer structure"]
impl crate::Writable for SdpmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDPMAX to value 0"]
impl crate::Resettable for SdpmaxSpec {
    const RESET_VALUE: u16 = 0;
}
