#[doc = "Register `SDPMIN` reader"]
pub type R = crate::R<SdpminSpec>;
#[doc = "Register `SDPMIN` writer"]
pub type W = crate::W<SdpminSpec>;
#[doc = "Field `SDMIN` reader - Set the minimum width of the special data pattern"]
pub type SdminR = crate::FieldReader<u16>;
#[doc = "Field `SDMIN` writer - Set the minimum width of the special data pattern"]
pub type SdminW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Set the minimum width of the special data pattern"]
    #[inline(always)]
    pub fn sdmin(&self) -> SdminR {
        SdminR::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the minimum width of the special data pattern"]
    #[inline(always)]
    pub fn sdmin(&mut self) -> SdminW<SdpminSpec> {
        SdminW::new(self, 0)
    }
}
#[doc = "Special Data Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdpminSpec;
impl crate::RegisterSpec for SdpminSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdpmin::R`](R) reader structure"]
impl crate::Readable for SdpminSpec {}
#[doc = "`write(|w| ..)` method takes [`sdpmin::W`](W) writer structure"]
impl crate::Writable for SdpminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDPMIN to value 0"]
impl crate::Resettable for SdpminSpec {
    const RESET_VALUE: u16 = 0;
}
