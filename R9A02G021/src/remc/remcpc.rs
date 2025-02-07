#[doc = "Register `REMCPC` reader"]
pub type R = crate::R<RemcpcSpec>;
#[doc = "Register `REMCPC` writer"]
pub type W = crate::W<RemcpcSpec>;
#[doc = "Field `CPN` reader - Compare Bit Count Specification"]
pub type CpnR = crate::FieldReader;
#[doc = "Field `CPN` writer - Compare Bit Count Specification"]
pub type CpnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Compare Bit Count Specification"]
    #[inline(always)]
    pub fn cpn(&self) -> CpnR {
        CpnR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compare Bit Count Specification"]
    #[inline(always)]
    pub fn cpn(&mut self) -> CpnW<RemcpcSpec> {
        CpnW::new(self, 0)
    }
}
#[doc = "Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remcpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemcpcSpec;
impl crate::RegisterSpec for RemcpcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remcpc::R`](R) reader structure"]
impl crate::Readable for RemcpcSpec {}
#[doc = "`write(|w| ..)` method takes [`remcpc::W`](W) writer structure"]
impl crate::Writable for RemcpcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMCPC to value 0"]
impl crate::Resettable for RemcpcSpec {
    const RESET_VALUE: u8 = 0;
}
