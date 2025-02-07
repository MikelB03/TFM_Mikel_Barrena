#[doc = "Register `DBGSTR` reader"]
pub type R = crate::R<DbgstrSpec>;
#[doc = "Field `DMACTIVE` reader - Debug active status from Debug Module"]
pub type DmactiveR = crate::BitReader;
impl R {
    #[doc = "Bit 28 - Debug active status from Debug Module"]
    #[inline(always)]
    pub fn dmactive(&self) -> DmactiveR {
        DmactiveR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Debug Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgstrSpec;
impl crate::RegisterSpec for DbgstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgstr::R`](R) reader structure"]
impl crate::Readable for DbgstrSpec {}
#[doc = "`reset()` method sets DBGSTR to value 0"]
impl crate::Resettable for DbgstrSpec {
    const RESET_VALUE: u32 = 0;
}
