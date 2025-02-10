#[doc = "Register `FWBL0` reader"]
pub type R = crate::R<Fwbl0Spec>;
#[doc = "Register `FWBL0` writer"]
pub type W = crate::W<Fwbl0Spec>;
#[doc = "Field `WDATA` reader - Flash Write Buffer L0"]
pub type WdataR = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - Flash Write Buffer L0"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Buffer L0"]
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Buffer L0"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<Fwbl0Spec> {
        WdataW::new(self, 0)
    }
}
#[doc = "Flash Write Buffer Register L0\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwbl0Spec;
impl crate::RegisterSpec for Fwbl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fwbl0::R`](R) reader structure"]
impl crate::Readable for Fwbl0Spec {}
#[doc = "`write(|w| ..)` method takes [`fwbl0::W`](W) writer structure"]
impl crate::Writable for Fwbl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FWBL0 to value 0"]
impl crate::Resettable for Fwbl0Spec {
    const RESET_VALUE: u16 = 0;
}
