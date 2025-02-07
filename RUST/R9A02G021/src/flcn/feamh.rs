#[doc = "Register `FEAMH` reader"]
pub type R = crate::R<FeamhSpec>;
#[doc = "Register `FEAMH` writer"]
pub type W = crate::W<FeamhSpec>;
#[doc = "Field `FEAMH` reader - Flash Error Address Monitor Register H"]
pub type FeamhR = crate::FieldReader<u16>;
#[doc = "Field `FEAMH` writer - Flash Error Address Monitor Register H"]
pub type FeamhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub fn feamh(&self) -> FeamhR {
        FeamhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub fn feamh(&mut self) -> FeamhW<FeamhSpec> {
        FeamhW::new(self, 0)
    }
}
#[doc = "Flash Error Address Monitor Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`feamh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feamh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeamhSpec;
impl crate::RegisterSpec for FeamhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`feamh::R`](R) reader structure"]
impl crate::Readable for FeamhSpec {}
#[doc = "`write(|w| ..)` method takes [`feamh::W`](W) writer structure"]
impl crate::Writable for FeamhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FEAMH to value 0"]
impl crate::Resettable for FeamhSpec {
    const RESET_VALUE: u16 = 0;
}
