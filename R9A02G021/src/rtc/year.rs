#[doc = "Register `YEAR` reader"]
pub type R = crate::R<YearSpec>;
#[doc = "Register `YEAR` writer"]
pub type W = crate::W<YearSpec>;
#[doc = "Field `YEAR1` reader - 1-year count"]
pub type Year1R = crate::FieldReader;
#[doc = "Field `YEAR1` writer - 1-year count"]
pub type Year1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR10` reader - 10-year count"]
pub type Year10R = crate::FieldReader;
#[doc = "Field `YEAR10` writer - 10-year count"]
pub type Year10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1-year count"]
    #[inline(always)]
    pub fn year1(&self) -> Year1R {
        Year1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - 10-year count"]
    #[inline(always)]
    pub fn year10(&self) -> Year10R {
        Year10R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-year count"]
    #[inline(always)]
    pub fn year1(&mut self) -> Year1W<YearSpec> {
        Year1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 10-year count"]
    #[inline(always)]
    pub fn year10(&mut self) -> Year10W<YearSpec> {
        Year10W::new(self, 4)
    }
}
#[doc = "Year Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YearSpec;
impl crate::RegisterSpec for YearSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`year::R`](R) reader structure"]
impl crate::Readable for YearSpec {}
#[doc = "`write(|w| ..)` method takes [`year::W`](W) writer structure"]
impl crate::Writable for YearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets YEAR to value 0"]
impl crate::Resettable for YearSpec {
    const RESET_VALUE: u8 = 0;
}
