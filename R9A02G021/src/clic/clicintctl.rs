#[doc = "Register `clicintctl%s` reader"]
pub type R = crate::R<ClicintctlSpec>;
#[doc = "Register `clicintctl%s` writer"]
pub type W = crate::W<ClicintctlSpec>;
#[doc = "Field `LVL_PRIO` reader - Interrupt level and priority for the associated interrupt"]
pub type LvlPrioR = crate::FieldReader;
#[doc = "Field `LVL_PRIO` writer - Interrupt level and priority for the associated interrupt"]
pub type LvlPrioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - Interrupt level and priority for the associated interrupt"]
    #[inline(always)]
    pub fn lvl_prio(&self) -> LvlPrioR {
        LvlPrioR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 4:7 - Interrupt level and priority for the associated interrupt"]
    #[inline(always)]
    pub fn lvl_prio(&mut self) -> LvlPrioW<ClicintctlSpec> {
        LvlPrioW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicintctlSpec;
impl crate::RegisterSpec for ClicintctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintctl::R`](R) reader structure"]
impl crate::Readable for ClicintctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clicintctl::W`](W) writer structure"]
impl crate::Writable for ClicintctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets clicintctl%s to value 0x0f"]
impl crate::Resettable for ClicintctlSpec {
    const RESET_VALUE: u8 = 0x0f;
}
