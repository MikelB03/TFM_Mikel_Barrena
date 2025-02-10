#[doc = "Register `FEARH` reader"]
pub type R = crate::R<FearhSpec>;
#[doc = "Register `FEARH` writer"]
pub type W = crate::W<FearhSpec>;
#[doc = "Field `FEARH` reader - Flash Processing End Address H"]
pub type FearhR = crate::FieldReader<u16>;
#[doc = "Field `FEARH` writer - Flash Processing End Address H"]
pub type FearhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing End Address H"]
    #[inline(always)]
    pub fn fearh(&self) -> FearhR {
        FearhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing End Address H"]
    #[inline(always)]
    pub fn fearh(&mut self) -> FearhW<FearhSpec> {
        FearhW::new(self, 0)
    }
}
#[doc = "Flash Processing End Address Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`fearh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fearh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FearhSpec;
impl crate::RegisterSpec for FearhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fearh::R`](R) reader structure"]
impl crate::Readable for FearhSpec {}
#[doc = "`write(|w| ..)` method takes [`fearh::W`](W) writer structure"]
impl crate::Writable for FearhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FEARH to value 0"]
impl crate::Resettable for FearhSpec {
    const RESET_VALUE: u16 = 0;
}
