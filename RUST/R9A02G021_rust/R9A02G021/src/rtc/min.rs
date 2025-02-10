#[doc = "Register `MIN` reader"]
pub type R = crate::R<MinSpec>;
#[doc = "Register `MIN` writer"]
pub type W = crate::W<MinSpec>;
#[doc = "Field `MIN1` reader - 1-minute count"]
pub type Min1R = crate::FieldReader;
#[doc = "Field `MIN1` writer - 1-minute count"]
pub type Min1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIN10` reader - 10-minute count"]
pub type Min10R = crate::FieldReader;
#[doc = "Field `MIN10` writer - 10-minute count"]
pub type Min10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 1-minute count"]
    #[inline(always)]
    pub fn min1(&self) -> Min1R {
        Min1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-minute count"]
    #[inline(always)]
    pub fn min10(&self) -> Min10R {
        Min10R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-minute count"]
    #[inline(always)]
    pub fn min1(&mut self) -> Min1W<MinSpec> {
        Min1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 10-minute count"]
    #[inline(always)]
    pub fn min10(&mut self) -> Min10W<MinSpec> {
        Min10W::new(self, 4)
    }
}
#[doc = "Minute Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MinSpec;
impl crate::RegisterSpec for MinSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`min::R`](R) reader structure"]
impl crate::Readable for MinSpec {}
#[doc = "`write(|w| ..)` method takes [`min::W`](W) writer structure"]
impl crate::Writable for MinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MIN to value 0"]
impl crate::Resettable for MinSpec {
    const RESET_VALUE: u8 = 0;
}
