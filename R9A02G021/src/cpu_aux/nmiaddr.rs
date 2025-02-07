#[doc = "Register `NMIADDR` reader"]
pub type R = crate::R<NmiaddrSpec>;
#[doc = "Register `NMIADDR` writer"]
pub type W = crate::W<NmiaddrSpec>;
#[doc = "Field `NMIADDR` reader - NMI handler address. Lower 6 bits must always be set 0."]
pub type NmiaddrR = crate::FieldReader<u32>;
#[doc = "Field `NMIADDR` writer - NMI handler address. Lower 6 bits must always be set 0."]
pub type NmiaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NMI handler address. Lower 6 bits must always be set 0."]
    #[inline(always)]
    pub fn nmiaddr(&self) -> NmiaddrR {
        NmiaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NMI handler address. Lower 6 bits must always be set 0."]
    #[inline(always)]
    pub fn nmiaddr(&mut self) -> NmiaddrW<NmiaddrSpec> {
        NmiaddrW::new(self, 0)
    }
}
#[doc = "NMI Hander Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiaddrSpec;
impl crate::RegisterSpec for NmiaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmiaddr::R`](R) reader structure"]
impl crate::Readable for NmiaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmiaddr::W`](W) writer structure"]
impl crate::Writable for NmiaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMIADDR to value 0"]
impl crate::Resettable for NmiaddrSpec {
    const RESET_VALUE: u32 = 0;
}
