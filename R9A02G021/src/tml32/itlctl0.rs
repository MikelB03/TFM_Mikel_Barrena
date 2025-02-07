#[doc = "Register `ITLCTL0` reader"]
pub type R = crate::R<Itlctl0Spec>;
#[doc = "Register `ITLCTL0` writer"]
pub type W = crate::W<Itlctl0Spec>;
#[doc = "8-bit counter mode: ITL000 count enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En0 {
    #[doc = "0: Counting stops"]
    _0 = 0,
    #[doc = "1: Counting begins"]
    _1 = 1,
}
impl From<En0> for bool {
    #[inline(always)]
    fn from(variant: En0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN0` reader - 8-bit counter mode: ITL000 count enable"]
pub type En0R = crate::BitReader<En0>;
impl En0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En0 {
        match self.bits {
            false => En0::_0,
            true => En0::_1,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En0::_0
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En0::_1
    }
}
#[doc = "Field `EN0` writer - 8-bit counter mode: ITL000 count enable"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG, En0>;
impl<'a, REG> En0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En0::_0)
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En0::_1)
    }
}
#[doc = "8-bit counter mode: ITL001 count enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En1 {
    #[doc = "0: Counting stops"]
    _0 = 0,
    #[doc = "1: Counting begins"]
    _1 = 1,
}
impl From<En1> for bool {
    #[inline(always)]
    fn from(variant: En1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - 8-bit counter mode: ITL001 count enable"]
pub type En1R = crate::BitReader<En1>;
impl En1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En1 {
        match self.bits {
            false => En1::_0,
            true => En1::_1,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En1::_0
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En1::_1
    }
}
#[doc = "Field `EN1` writer - 8-bit counter mode: ITL001 count enable"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG, En1>;
impl<'a, REG> En1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En1::_0)
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En1::_1)
    }
}
#[doc = "8-bit counter mode: ITL012 count enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En2 {
    #[doc = "0: Counting stops"]
    _0 = 0,
    #[doc = "1: Counting begins"]
    _1 = 1,
}
impl From<En2> for bool {
    #[inline(always)]
    fn from(variant: En2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN2` reader - 8-bit counter mode: ITL012 count enable"]
pub type En2R = crate::BitReader<En2>;
impl En2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En2 {
        match self.bits {
            false => En2::_0,
            true => En2::_1,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En2::_0
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En2::_1
    }
}
#[doc = "Field `EN2` writer - 8-bit counter mode: ITL012 count enable"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG, En2>;
impl<'a, REG> En2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En2::_0)
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En2::_1)
    }
}
#[doc = "8-bit counter mode: ITL013 count enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En3 {
    #[doc = "0: Counting stops"]
    _0 = 0,
    #[doc = "1: Counting begins"]
    _1 = 1,
}
impl From<En3> for bool {
    #[inline(always)]
    fn from(variant: En3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN3` reader - 8-bit counter mode: ITL013 count enable"]
pub type En3R = crate::BitReader<En3>;
impl En3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En3 {
        match self.bits {
            false => En3::_0,
            true => En3::_1,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En3::_0
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En3::_1
    }
}
#[doc = "Field `EN3` writer - 8-bit counter mode: ITL013 count enable"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG, En3>;
impl<'a, REG> En3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En3::_0)
    }
    #[doc = "Counting begins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En3::_1)
    }
}
#[doc = "Selection of 8-bit, 16-bit, or 32-bit counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md0 {
    #[doc = "0: The interval timer operates in 8-bit counter mode."]
    _00 = 0,
    #[doc = "1: The interval timer operates in 16-bit counter mode (channel 0 is connected with channel 1 and channel 2 is connected with channel 3)."]
    _01 = 1,
    #[doc = "2: The interval timer operates in 32-bit counter mode (channels 0 to 3 are connected)."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<Md0> for u8 {
    #[inline(always)]
    fn from(variant: Md0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md0 {
    type Ux = u8;
}
impl crate::IsEnum for Md0 {}
#[doc = "Field `MD0` reader - Selection of 8-bit, 16-bit, or 32-bit counter mode"]
pub type Md0R = crate::FieldReader<Md0>;
impl Md0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md0 {
        match self.bits {
            0 => Md0::_00,
            1 => Md0::_01,
            2 => Md0::_10,
            3 => Md0::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The interval timer operates in 8-bit counter mode."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Md0::_00
    }
    #[doc = "The interval timer operates in 16-bit counter mode (channel 0 is connected with channel 1 and channel 2 is connected with channel 3)."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Md0::_01
    }
    #[doc = "The interval timer operates in 32-bit counter mode (channels 0 to 3 are connected)."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Md0::_10
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Md0::_11
    }
}
#[doc = "Field `MD0` writer - Selection of 8-bit, 16-bit, or 32-bit counter mode"]
pub type Md0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Md0, crate::Safe>;
impl<'a, REG> Md0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The interval timer operates in 8-bit counter mode."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_00)
    }
    #[doc = "The interval timer operates in 16-bit counter mode (channel 0 is connected with channel 1 and channel 2 is connected with channel 3)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_01)
    }
    #[doc = "The interval timer operates in 32-bit counter mode (channels 0 to 3 are connected)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_11)
    }
}
impl R {
    #[doc = "Bit 0 - 8-bit counter mode: ITL000 count enable"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit counter mode: ITL001 count enable"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit counter mode: ITL012 count enable"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit counter mode: ITL013 count enable"]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Selection of 8-bit, 16-bit, or 32-bit counter mode"]
    #[inline(always)]
    pub fn md0(&self) -> Md0R {
        Md0R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit counter mode: ITL000 count enable"]
    #[inline(always)]
    pub fn en0(&mut self) -> En0W<Itlctl0Spec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 1 - 8-bit counter mode: ITL001 count enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<Itlctl0Spec> {
        En1W::new(self, 1)
    }
    #[doc = "Bit 2 - 8-bit counter mode: ITL012 count enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<Itlctl0Spec> {
        En2W::new(self, 2)
    }
    #[doc = "Bit 3 - 8-bit counter mode: ITL013 count enable"]
    #[inline(always)]
    pub fn en3(&mut self) -> En3W<Itlctl0Spec> {
        En3W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Selection of 8-bit, 16-bit, or 32-bit counter mode"]
    #[inline(always)]
    pub fn md0(&mut self) -> Md0W<Itlctl0Spec> {
        Md0W::new(self, 6)
    }
}
#[doc = "Interval Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itlctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlctl0Spec;
impl crate::RegisterSpec for Itlctl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlctl0::R`](R) reader structure"]
impl crate::Readable for Itlctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`itlctl0::W`](W) writer structure"]
impl crate::Writable for Itlctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLCTL0 to value 0"]
impl crate::Resettable for Itlctl0Spec {
    const RESET_VALUE: u8 = 0;
}
