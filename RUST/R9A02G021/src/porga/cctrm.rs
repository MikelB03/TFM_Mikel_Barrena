#[doc = "Register `CCTRM` reader"]
pub type R = crate::R<CctrmSpec>;
#[doc = "Register `CCTRM` writer"]
pub type W = crate::W<CctrmSpec>;
#[doc = "Field `IADJ` reader - Output current control trimming"]
pub type IadjR = crate::FieldReader;
#[doc = "Field `IADJ` writer - Output current control trimming"]
pub type IadjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Output current control trimming"]
    #[inline(always)]
    pub fn iadj(&self) -> IadjR {
        IadjR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output current control trimming"]
    #[inline(always)]
    pub fn iadj(&mut self) -> IadjW<CctrmSpec> {
        IadjW::new(self, 0)
    }
}
#[doc = "Output Current Control Trimming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CctrmSpec;
impl crate::RegisterSpec for CctrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cctrm::R`](R) reader structure"]
impl crate::Readable for CctrmSpec {}
#[doc = "`write(|w| ..)` method takes [`cctrm::W`](W) writer structure"]
impl crate::Writable for CctrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CCTRM to value 0"]
impl crate::Resettable for CctrmSpec {
    const RESET_VALUE: u8 = 0;
}
