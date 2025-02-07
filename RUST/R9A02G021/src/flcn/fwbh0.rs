#[doc = "Register `FWBH0` reader"]
pub type R = crate::R<Fwbh0Spec>;
#[doc = "Register `FWBH0` writer"]
pub type W = crate::W<Fwbh0Spec>;
#[doc = "Field `WDATA` reader - Flash Write Buffer H0"]
pub type WdataR = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - Flash Write Buffer H0"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Buffer H0"]
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Buffer H0"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<Fwbh0Spec> {
        WdataW::new(self, 0)
    }
}
#[doc = "Flash Write Buffer Register H0\n\nYou can [`read`](crate::Reg::read) this register and get [`fwbh0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwbh0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwbh0Spec;
impl crate::RegisterSpec for Fwbh0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fwbh0::R`](R) reader structure"]
impl crate::Readable for Fwbh0Spec {}
#[doc = "`write(|w| ..)` method takes [`fwbh0::W`](W) writer structure"]
impl crate::Writable for Fwbh0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FWBH0 to value 0"]
impl crate::Resettable for Fwbh0Spec {
    const RESET_VALUE: u16 = 0;
}
