#[doc = "Register `ELSR%s` reader"]
pub type R = crate::R<ElsrSpec>;
#[doc = "Register `ELSR%s` writer"]
pub type W = crate::W<ElsrSpec>;
#[doc = "Field `ELS` reader - Event Link Select"]
pub type ElsR = crate::FieldReader;
#[doc = "Field `ELS` writer - Event Link Select"]
pub type ElsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    pub fn els(&self) -> ElsR {
        ElsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    pub fn els(&mut self) -> ElsW<ElsrSpec> {
        ElsW::new(self, 0)
    }
}
#[doc = "Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElsrSpec;
impl crate::RegisterSpec for ElsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`elsr::R`](R) reader structure"]
impl crate::Readable for ElsrSpec {}
#[doc = "`write(|w| ..)` method takes [`elsr::W`](W) writer structure"]
impl crate::Writable for ElsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ELSR%s to value 0"]
impl crate::Resettable for ElsrSpec {
    const RESET_VALUE: u16 = 0;
}
