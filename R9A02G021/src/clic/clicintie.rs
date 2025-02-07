#[doc = "Register `clicintie%s` reader"]
pub type R = crate::R<ClicintieSpec>;
#[doc = "Register `clicintie%s` writer"]
pub type W = crate::W<ClicintieSpec>;
#[doc = "Field `IE` reader - Enable bit for the associated interrupt"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Enable bit for the associated interrupt"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable bit for the associated interrupt"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit for the associated interrupt"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<ClicintieSpec> {
        IeW::new(self, 0)
    }
}
#[doc = "CLIC Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicintieSpec;
impl crate::RegisterSpec for ClicintieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintie::R`](R) reader structure"]
impl crate::Readable for ClicintieSpec {}
#[doc = "`write(|w| ..)` method takes [`clicintie::W`](W) writer structure"]
impl crate::Writable for ClicintieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets clicintie%s to value 0"]
impl crate::Resettable for ClicintieSpec {
    const RESET_VALUE: u8 = 0;
}
