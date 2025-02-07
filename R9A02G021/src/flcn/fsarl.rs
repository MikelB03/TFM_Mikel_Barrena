#[doc = "Register `FSARL` reader"]
pub type R = crate::R<FsarlSpec>;
#[doc = "Register `FSARL` writer"]
pub type W = crate::W<FsarlSpec>;
#[doc = "Field `FSARL` reader - Flash Processing Start Address L"]
pub type FsarlR = crate::FieldReader<u16>;
#[doc = "Field `FSARL` writer - Flash Processing Start Address L"]
pub type FsarlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing Start Address L"]
    #[inline(always)]
    pub fn fsarl(&self) -> FsarlR {
        FsarlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing Start Address L"]
    #[inline(always)]
    pub fn fsarl(&mut self) -> FsarlW<FsarlSpec> {
        FsarlW::new(self, 0)
    }
}
#[doc = "Flash Processing Start Address Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`fsarl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsarl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsarlSpec;
impl crate::RegisterSpec for FsarlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fsarl::R`](R) reader structure"]
impl crate::Readable for FsarlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsarl::W`](W) writer structure"]
impl crate::Writable for FsarlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FSARL to value 0"]
impl crate::Resettable for FsarlSpec {
    const RESET_VALUE: u16 = 0;
}
