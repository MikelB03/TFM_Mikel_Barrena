#[doc = "Register `ITLFDIV01` reader"]
pub type R = crate::R<Itlfdiv01Spec>;
#[doc = "Register `ITLFDIV01` writer"]
pub type W = crate::W<Itlfdiv01Spec>;
#[doc = "8-bit counter mode: Counter clock for ITL012\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdiv2 {
    #[doc = "0: fITL0"]
    _000 = 0,
    #[doc = "1: fITL0/2"]
    _001 = 1,
    #[doc = "2: fITL0/4"]
    _010 = 2,
    #[doc = "3: fITL0/8"]
    _011 = 3,
    #[doc = "4: fITL0/16"]
    _100 = 4,
    #[doc = "5: fITL0/32"]
    _101 = 5,
    #[doc = "6: fITL0/64"]
    _110 = 6,
    #[doc = "7: fITL0/128"]
    _111 = 7,
}
impl From<Fdiv2> for u8 {
    #[inline(always)]
    fn from(variant: Fdiv2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdiv2 {
    type Ux = u8;
}
impl crate::IsEnum for Fdiv2 {}
#[doc = "Field `FDIV2` reader - 8-bit counter mode: Counter clock for ITL012"]
pub type Fdiv2R = crate::FieldReader<Fdiv2>;
impl Fdiv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdiv2 {
        match self.bits {
            0 => Fdiv2::_000,
            1 => Fdiv2::_001,
            2 => Fdiv2::_010,
            3 => Fdiv2::_011,
            4 => Fdiv2::_100,
            5 => Fdiv2::_101,
            6 => Fdiv2::_110,
            7 => Fdiv2::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fdiv2::_000
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fdiv2::_001
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fdiv2::_010
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fdiv2::_011
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fdiv2::_100
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fdiv2::_101
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fdiv2::_110
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fdiv2::_111
    }
}
#[doc = "Field `FDIV2` writer - 8-bit counter mode: Counter clock for ITL012"]
pub type Fdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdiv2, crate::Safe>;
impl<'a, REG> Fdiv2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_000)
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_001)
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_010)
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_011)
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_100)
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_101)
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_110)
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv2::_111)
    }
}
#[doc = "8-bit counter mode: Counter clock for ITL013\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdiv3 {
    #[doc = "0: fITL0"]
    _000 = 0,
    #[doc = "1: fITL0/2"]
    _001 = 1,
    #[doc = "2: fITL0/4"]
    _010 = 2,
    #[doc = "3: fITL0/8"]
    _011 = 3,
    #[doc = "4: fITL0/16"]
    _100 = 4,
    #[doc = "5: fITL0/32"]
    _101 = 5,
    #[doc = "6: fITL0/64"]
    _110 = 6,
    #[doc = "7: fITL0/128"]
    _111 = 7,
}
impl From<Fdiv3> for u8 {
    #[inline(always)]
    fn from(variant: Fdiv3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdiv3 {
    type Ux = u8;
}
impl crate::IsEnum for Fdiv3 {}
#[doc = "Field `FDIV3` reader - 8-bit counter mode: Counter clock for ITL013"]
pub type Fdiv3R = crate::FieldReader<Fdiv3>;
impl Fdiv3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdiv3 {
        match self.bits {
            0 => Fdiv3::_000,
            1 => Fdiv3::_001,
            2 => Fdiv3::_010,
            3 => Fdiv3::_011,
            4 => Fdiv3::_100,
            5 => Fdiv3::_101,
            6 => Fdiv3::_110,
            7 => Fdiv3::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fdiv3::_000
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fdiv3::_001
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fdiv3::_010
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fdiv3::_011
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fdiv3::_100
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fdiv3::_101
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fdiv3::_110
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fdiv3::_111
    }
}
#[doc = "Field `FDIV3` writer - 8-bit counter mode: Counter clock for ITL013"]
pub type Fdiv3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdiv3, crate::Safe>;
impl<'a, REG> Fdiv3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_000)
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_001)
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_010)
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_011)
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_100)
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_101)
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_110)
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv3::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - 8-bit counter mode: Counter clock for ITL012"]
    #[inline(always)]
    pub fn fdiv2(&self) -> Fdiv2R {
        Fdiv2R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - 8-bit counter mode: Counter clock for ITL013"]
    #[inline(always)]
    pub fn fdiv3(&self) -> Fdiv3R {
        Fdiv3R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - 8-bit counter mode: Counter clock for ITL012"]
    #[inline(always)]
    pub fn fdiv2(&mut self) -> Fdiv2W<Itlfdiv01Spec> {
        Fdiv2W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 8-bit counter mode: Counter clock for ITL013"]
    #[inline(always)]
    pub fn fdiv3(&mut self) -> Fdiv3W<Itlfdiv01Spec> {
        Fdiv3W::new(self, 4)
    }
}
#[doc = "Interval Timer Frequency Division Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itlfdiv01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlfdiv01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlfdiv01Spec;
impl crate::RegisterSpec for Itlfdiv01Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlfdiv01::R`](R) reader structure"]
impl crate::Readable for Itlfdiv01Spec {}
#[doc = "`write(|w| ..)` method takes [`itlfdiv01::W`](W) writer structure"]
impl crate::Writable for Itlfdiv01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLFDIV01 to value 0"]
impl crate::Resettable for Itlfdiv01Spec {
    const RESET_VALUE: u8 = 0;
}
