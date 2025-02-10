#[doc = "Register `FSARH` reader"]
pub type R = crate::R<FsarhSpec>;
#[doc = "Register `FSARH` writer"]
pub type W = crate::W<FsarhSpec>;
#[doc = "Field `FSARH` reader - Flash Processing Start Address H"]
pub type FsarhR = crate::FieldReader<u16>;
#[doc = "Field `FSARH` writer - Flash Processing Start Address H"]
pub type FsarhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing Start Address H"]
    #[inline(always)]
    pub fn fsarh(&self) -> FsarhR {
        FsarhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing Start Address H"]
    #[inline(always)]
    pub fn fsarh(&mut self) -> FsarhW<FsarhSpec> {
        FsarhW::new(self, 0)
    }
}
#[doc = "Flash Processing Start Address Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`fsarh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsarh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsarhSpec;
impl crate::RegisterSpec for FsarhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fsarh::R`](R) reader structure"]
impl crate::Readable for FsarhSpec {}
#[doc = "`write(|w| ..)` method takes [`fsarh::W`](W) writer structure"]
impl crate::Writable for FsarhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FSARH to value 0"]
impl crate::Resettable for FsarhSpec {
    const RESET_VALUE: u16 = 0;
}
