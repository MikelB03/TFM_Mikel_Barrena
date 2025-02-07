#[doc = "Register `ITLCMP0%s` reader"]
pub type R = crate::R<Itlcmp0Spec>;
#[doc = "Register `ITLCMP0%s` writer"]
pub type W = crate::W<Itlcmp0Spec>;
#[doc = "Field `CMP16` reader - 16-bit timer setting compare data for unit n"]
pub type Cmp16R = crate::FieldReader<u16>;
#[doc = "Field `CMP16` writer - 16-bit timer setting compare data for unit n"]
pub type Cmp16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit timer setting compare data for unit n"]
    #[inline(always)]
    pub fn cmp16(&self) -> Cmp16R {
        Cmp16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit timer setting compare data for unit n"]
    #[inline(always)]
    pub fn cmp16(&mut self) -> Cmp16W<Itlcmp0Spec> {
        Cmp16W::new(self, 0)
    }
}
#[doc = "Interval Timer Compare Registers 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlcmp0Spec;
impl crate::RegisterSpec for Itlcmp0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`itlcmp0::R`](R) reader structure"]
impl crate::Readable for Itlcmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`itlcmp0::W`](W) writer structure"]
impl crate::Writable for Itlcmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ITLCMP0%s to value 0xffff"]
impl crate::Resettable for Itlcmp0Spec {
    const RESET_VALUE: u16 = 0xffff;
}
