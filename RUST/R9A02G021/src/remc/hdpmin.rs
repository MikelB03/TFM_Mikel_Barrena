#[doc = "Register `HDPMIN` reader"]
pub type R = crate::R<HdpminSpec>;
#[doc = "Register `HDPMIN` writer"]
pub type W = crate::W<HdpminSpec>;
#[doc = "Field `HDMIN` reader - Set the minimum width of the header pattern"]
pub type HdminR = crate::FieldReader<u16>;
#[doc = "Field `HDMIN` writer - Set the minimum width of the header pattern"]
pub type HdminW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Set the minimum width of the header pattern"]
    #[inline(always)]
    pub fn hdmin(&self) -> HdminR {
        HdminR::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the minimum width of the header pattern"]
    #[inline(always)]
    pub fn hdmin(&mut self) -> HdminW<HdpminSpec> {
        HdminW::new(self, 0)
    }
}
#[doc = "Header Pattern Minimum Width Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdpminSpec;
impl crate::RegisterSpec for HdpminSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hdpmin::R`](R) reader structure"]
impl crate::Readable for HdpminSpec {}
#[doc = "`write(|w| ..)` method takes [`hdpmin::W`](W) writer structure"]
impl crate::Writable for HdpminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HDPMIN to value 0"]
impl crate::Resettable for HdpminSpec {
    const RESET_VALUE: u16 = 0;
}
