#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `SEC1` reader - 1-second count"]
pub type Sec1R = crate::FieldReader;
#[doc = "Field `SEC1` writer - 1-second count"]
pub type Sec1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEC10` reader - 10-second count"]
pub type Sec10R = crate::FieldReader;
#[doc = "Field `SEC10` writer - 10-second count"]
pub type Sec10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 1-second count"]
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-second count"]
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-second count"]
    #[inline(always)]
    pub fn sec1(&mut self) -> Sec1W<SecSpec> {
        Sec1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 10-second count"]
    #[inline(always)]
    pub fn sec10(&mut self) -> Sec10W<SecSpec> {
        Sec10W::new(self, 4)
    }
}
#[doc = "Second Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {
    const RESET_VALUE: u8 = 0;
}
