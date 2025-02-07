#[doc = "Register `MONTH` reader"]
pub type R = crate::R<MonthSpec>;
#[doc = "Register `MONTH` writer"]
pub type W = crate::W<MonthSpec>;
#[doc = "Field `MONTH1` reader - 1-month count"]
pub type Month1R = crate::FieldReader;
#[doc = "Field `MONTH1` writer - 1-month count"]
pub type Month1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTH10` reader - 10-month count"]
pub type Month10R = crate::BitReader;
#[doc = "Field `MONTH10` writer - 10-month count"]
pub type Month10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 1-month count"]
    #[inline(always)]
    pub fn month1(&self) -> Month1R {
        Month1R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-month count"]
    #[inline(always)]
    pub fn month10(&self) -> Month10R {
        Month10R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-month count"]
    #[inline(always)]
    pub fn month1(&mut self) -> Month1W<MonthSpec> {
        Month1W::new(self, 0)
    }
    #[doc = "Bit 4 - 10-month count"]
    #[inline(always)]
    pub fn month10(&mut self) -> Month10W<MonthSpec> {
        Month10W::new(self, 4)
    }
}
#[doc = "Month Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`month::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`month::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonthSpec;
impl crate::RegisterSpec for MonthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`month::R`](R) reader structure"]
impl crate::Readable for MonthSpec {}
#[doc = "`write(|w| ..)` method takes [`month::W`](W) writer structure"]
impl crate::Writable for MonthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MONTH to value 0"]
impl crate::Resettable for MonthSpec {
    const RESET_VALUE: u8 = 0;
}
