#[doc = "Register `CRCSAR` reader"]
pub type R = crate::R<CrcsarSpec>;
#[doc = "Register `CRCSAR` writer"]
pub type W = crate::W<CrcsarSpec>;
#[doc = "Field `CRCSA` reader - Register Snoop Address"]
pub type CrcsaR = crate::FieldReader<u16>;
#[doc = "Field `CRCSA` writer - Register Snoop Address"]
pub type CrcsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Register Snoop Address"]
    #[inline(always)]
    pub fn crcsa(&self) -> CrcsaR {
        CrcsaR::new(self.bits & 0x3fff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Register Snoop Address"]
    #[inline(always)]
    pub fn crcsa(&mut self) -> CrcsaW<CrcsarSpec> {
        CrcsaW::new(self, 0)
    }
}
#[doc = "Snoop Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcsarSpec;
impl crate::RegisterSpec for CrcsarSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcsar::R`](R) reader structure"]
impl crate::Readable for CrcsarSpec {}
#[doc = "`write(|w| ..)` method takes [`crcsar::W`](W) writer structure"]
impl crate::Writable for CrcsarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRCSAR to value 0"]
impl crate::Resettable for CrcsarSpec {
    const RESET_VALUE: u16 = 0;
}
