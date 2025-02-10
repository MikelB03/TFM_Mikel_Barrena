#[doc = "Register `FEARL` reader"]
pub type R = crate::R<FearlSpec>;
#[doc = "Register `FEARL` writer"]
pub type W = crate::W<FearlSpec>;
#[doc = "Field `FEARL` reader - Flash Processing End Address L"]
pub type FearlR = crate::FieldReader<u16>;
#[doc = "Field `FEARL` writer - Flash Processing End Address L"]
pub type FearlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing End Address L"]
    #[inline(always)]
    pub fn fearl(&self) -> FearlR {
        FearlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing End Address L"]
    #[inline(always)]
    pub fn fearl(&mut self) -> FearlW<FearlSpec> {
        FearlW::new(self, 0)
    }
}
#[doc = "Flash Processing End Address Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`fearl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fearl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FearlSpec;
impl crate::RegisterSpec for FearlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fearl::R`](R) reader structure"]
impl crate::Readable for FearlSpec {}
#[doc = "`write(|w| ..)` method takes [`fearl::W`](W) writer structure"]
impl crate::Writable for FearlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FEARL to value 0"]
impl crate::Resettable for FearlSpec {
    const RESET_VALUE: u16 = 0;
}
