#[doc = "Register `HOUR` reader"]
pub type R = crate::R<HourSpec>;
#[doc = "Register `HOUR` writer"]
pub type W = crate::W<HourSpec>;
#[doc = "Field `HOUR1` reader - 1-hour count"]
pub type Hour1R = crate::FieldReader;
#[doc = "Field `HOUR1` writer - 1-hour count"]
pub type Hour1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOUR10` reader - 10-hour count"]
pub type Hour10R = crate::FieldReader;
#[doc = "Field `HOUR10` writer - 10-hour count"]
pub type Hour10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 1-hour count"]
    #[inline(always)]
    pub fn hour1(&self) -> Hour1R {
        Hour1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-hour count"]
    #[inline(always)]
    pub fn hour10(&self) -> Hour10R {
        Hour10R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-hour count"]
    #[inline(always)]
    pub fn hour1(&mut self) -> Hour1W<HourSpec> {
        Hour1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10-hour count"]
    #[inline(always)]
    pub fn hour10(&mut self) -> Hour10W<HourSpec> {
        Hour10W::new(self, 4)
    }
}
#[doc = "Hour Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HourSpec;
impl crate::RegisterSpec for HourSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hour::R`](R) reader structure"]
impl crate::Readable for HourSpec {}
#[doc = "`write(|w| ..)` method takes [`hour::W`](W) writer structure"]
impl crate::Writable for HourSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOUR to value 0"]
impl crate::Resettable for HourSpec {
    const RESET_VALUE: u8 = 0;
}
