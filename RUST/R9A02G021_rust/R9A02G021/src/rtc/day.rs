#[doc = "Register `DAY` reader"]
pub type R = crate::R<DaySpec>;
#[doc = "Register `DAY` writer"]
pub type W = crate::W<DaySpec>;
#[doc = "Field `DAY1` reader - 1-day count"]
pub type Day1R = crate::FieldReader;
#[doc = "Field `DAY1` writer - 1-day count"]
pub type Day1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAY10` reader - 10-day count"]
pub type Day10R = crate::FieldReader;
#[doc = "Field `DAY10` writer - 10-day count"]
pub type Day10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 1-day count"]
    #[inline(always)]
    pub fn day1(&self) -> Day1R {
        Day1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-day count"]
    #[inline(always)]
    pub fn day10(&self) -> Day10R {
        Day10R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-day count"]
    #[inline(always)]
    pub fn day1(&mut self) -> Day1W<DaySpec> {
        Day1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10-day count"]
    #[inline(always)]
    pub fn day10(&mut self) -> Day10W<DaySpec> {
        Day10W::new(self, 4)
    }
}
#[doc = "Day Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaySpec;
impl crate::RegisterSpec for DaySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`day::R`](R) reader structure"]
impl crate::Readable for DaySpec {}
#[doc = "`write(|w| ..)` method takes [`day::W`](W) writer structure"]
impl crate::Writable for DaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DAY to value 0"]
impl crate::Resettable for DaySpec {
    const RESET_VALUE: u8 = 0;
}
