#[doc = "Register `FWBL1` reader"]
pub type R = crate::R<Fwbl1Spec>;
#[doc = "Register `FWBL1` writer"]
pub type W = crate::W<Fwbl1Spec>;
#[doc = "Field `WDATA` reader - Flash Write Buffer L1 Bits \\[47:32\\]"]
pub type WdataR = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - Flash Write Buffer L1 Bits \\[47:32\\]"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Buffer L1 Bits \\[47:32\\]"]
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Buffer L1 Bits \\[47:32\\]"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<Fwbl1Spec> {
        WdataW::new(self, 0)
    }
}
#[doc = "Flash Write Buffer Register L1\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwbl1Spec;
impl crate::RegisterSpec for Fwbl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fwbl1::R`](R) reader structure"]
impl crate::Readable for Fwbl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fwbl1::W`](W) writer structure"]
impl crate::Writable for Fwbl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FWBL1 to value 0"]
impl crate::Resettable for Fwbl1Spec {
    const RESET_VALUE: u16 = 0;
}
