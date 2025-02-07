#[doc = "Register `REMPE` reader"]
pub type R = crate::R<RempeSpec>;
#[doc = "Register `REMPE` writer"]
pub type W = crate::W<RempeSpec>;
#[doc = "Field `PE` reader - Set the width of the pattern end"]
pub type PeR = crate::FieldReader<u16>;
#[doc = "Field `PE` writer - Set the width of the pattern end"]
pub type PeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Set the width of the pattern end"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the width of the pattern end"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<RempeSpec> {
        PeW::new(self, 0)
    }
}
#[doc = "Pattern End Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rempe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rempe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RempeSpec;
impl crate::RegisterSpec for RempeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rempe::R`](R) reader structure"]
impl crate::Readable for RempeSpec {}
#[doc = "`write(|w| ..)` method takes [`rempe::W`](W) writer structure"]
impl crate::Writable for RempeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REMPE to value 0"]
impl crate::Resettable for RempeSpec {
    const RESET_VALUE: u16 = 0;
}
