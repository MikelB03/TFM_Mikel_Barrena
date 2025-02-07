#[doc = "Register `SPS0` reader"]
pub type R = crate::R<Sps0Spec>;
#[doc = "Register `SPS0` writer"]
pub type W = crate::W<Sps0Spec>;
#[doc = "Selection of operation clock (CKm0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prs0 {
    #[doc = "0: PCLKB"]
    _0x0 = 0,
    #[doc = "1: PCLKB/2"]
    _0x1 = 1,
    #[doc = "2: PCLKB/22"]
    _0x2 = 2,
    #[doc = "3: PCLKB/23"]
    _0x3 = 3,
    #[doc = "4: PCLKB/24"]
    _0x4 = 4,
    #[doc = "5: PCLKB/25"]
    _0x5 = 5,
    #[doc = "6: PCLKB/26"]
    _0x6 = 6,
    #[doc = "7: PCLKB/27"]
    _0x7 = 7,
    #[doc = "8: PCLKB/28"]
    _0x8 = 8,
    #[doc = "9: PCLKB/29"]
    _0x9 = 9,
    #[doc = "10: PCLKB/210"]
    _0xA = 10,
    #[doc = "11: PCLKB/211"]
    _0xB = 11,
    #[doc = "12: PCLKB/212"]
    _0xC = 12,
    #[doc = "13: PCLKB/213"]
    _0xD = 13,
    #[doc = "14: PCLKB/214"]
    _0xE = 14,
    #[doc = "15: PCLKB/215"]
    _0xF = 15,
}
impl From<Prs0> for u8 {
    #[inline(always)]
    fn from(variant: Prs0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prs0 {
    type Ux = u8;
}
impl crate::IsEnum for Prs0 {}
#[doc = "Field `PRS0` reader - Selection of operation clock (CKm0)"]
pub type Prs0R = crate::FieldReader<Prs0>;
impl Prs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prs0 {
        match self.bits {
            0 => Prs0::_0x0,
            1 => Prs0::_0x1,
            2 => Prs0::_0x2,
            3 => Prs0::_0x3,
            4 => Prs0::_0x4,
            5 => Prs0::_0x5,
            6 => Prs0::_0x6,
            7 => Prs0::_0x7,
            8 => Prs0::_0x8,
            9 => Prs0::_0x9,
            10 => Prs0::_0xA,
            11 => Prs0::_0xB,
            12 => Prs0::_0xC,
            13 => Prs0::_0xD,
            14 => Prs0::_0xE,
            15 => Prs0::_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Prs0::_0x0
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Prs0::_0x1
    }
    #[doc = "PCLKB/22"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Prs0::_0x2
    }
    #[doc = "PCLKB/23"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Prs0::_0x3
    }
    #[doc = "PCLKB/24"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Prs0::_0x4
    }
    #[doc = "PCLKB/25"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Prs0::_0x5
    }
    #[doc = "PCLKB/26"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Prs0::_0x6
    }
    #[doc = "PCLKB/27"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Prs0::_0x7
    }
    #[doc = "PCLKB/28"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Prs0::_0x8
    }
    #[doc = "PCLKB/29"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Prs0::_0x9
    }
    #[doc = "PCLKB/210"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Prs0::_0xA
    }
    #[doc = "PCLKB/211"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Prs0::_0xB
    }
    #[doc = "PCLKB/212"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Prs0::_0xC
    }
    #[doc = "PCLKB/213"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Prs0::_0xD
    }
    #[doc = "PCLKB/214"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Prs0::_0xE
    }
    #[doc = "PCLKB/215"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Prs0::_0xF
    }
}
#[doc = "Field `PRS0` writer - Selection of operation clock (CKm0)"]
pub type Prs0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Prs0, crate::Safe>;
impl<'a, REG> Prs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x0)
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x1)
    }
    #[doc = "PCLKB/22"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x2)
    }
    #[doc = "PCLKB/23"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x3)
    }
    #[doc = "PCLKB/24"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x4)
    }
    #[doc = "PCLKB/25"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x5)
    }
    #[doc = "PCLKB/26"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x6)
    }
    #[doc = "PCLKB/27"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x7)
    }
    #[doc = "PCLKB/28"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x8)
    }
    #[doc = "PCLKB/29"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0x9)
    }
    #[doc = "PCLKB/210"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xA)
    }
    #[doc = "PCLKB/211"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xB)
    }
    #[doc = "PCLKB/212"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xC)
    }
    #[doc = "PCLKB/213"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xD)
    }
    #[doc = "PCLKB/214"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xE)
    }
    #[doc = "PCLKB/215"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Prs0::_0xF)
    }
}
#[doc = "Selection of operation clock (CKm1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prs1 {
    #[doc = "0: PCLKB"]
    _0x0 = 0,
    #[doc = "1: PCLKB/2"]
    _0x1 = 1,
    #[doc = "2: PCLKB/22"]
    _0x2 = 2,
    #[doc = "3: PCLKB/23"]
    _0x3 = 3,
    #[doc = "4: PCLKB/24"]
    _0x4 = 4,
    #[doc = "5: PCLKB/25"]
    _0x5 = 5,
    #[doc = "6: PCLKB/26"]
    _0x6 = 6,
    #[doc = "7: PCLKB/27"]
    _0x7 = 7,
    #[doc = "8: PCLKB/28"]
    _0x8 = 8,
    #[doc = "9: PCLKB/29"]
    _0x9 = 9,
    #[doc = "10: PCLKB/210"]
    _0xA = 10,
    #[doc = "11: PCLKB/211"]
    _0xB = 11,
    #[doc = "12: PCLKB/212"]
    _0xC = 12,
    #[doc = "13: PCLKB/213"]
    _0xD = 13,
    #[doc = "14: PCLKB/214"]
    _0xE = 14,
    #[doc = "15: PCLKB/215"]
    _0xF = 15,
}
impl From<Prs1> for u8 {
    #[inline(always)]
    fn from(variant: Prs1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prs1 {
    type Ux = u8;
}
impl crate::IsEnum for Prs1 {}
#[doc = "Field `PRS1` reader - Selection of operation clock (CKm1)"]
pub type Prs1R = crate::FieldReader<Prs1>;
impl Prs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prs1 {
        match self.bits {
            0 => Prs1::_0x0,
            1 => Prs1::_0x1,
            2 => Prs1::_0x2,
            3 => Prs1::_0x3,
            4 => Prs1::_0x4,
            5 => Prs1::_0x5,
            6 => Prs1::_0x6,
            7 => Prs1::_0x7,
            8 => Prs1::_0x8,
            9 => Prs1::_0x9,
            10 => Prs1::_0xA,
            11 => Prs1::_0xB,
            12 => Prs1::_0xC,
            13 => Prs1::_0xD,
            14 => Prs1::_0xE,
            15 => Prs1::_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Prs1::_0x0
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Prs1::_0x1
    }
    #[doc = "PCLKB/22"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Prs1::_0x2
    }
    #[doc = "PCLKB/23"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Prs1::_0x3
    }
    #[doc = "PCLKB/24"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Prs1::_0x4
    }
    #[doc = "PCLKB/25"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Prs1::_0x5
    }
    #[doc = "PCLKB/26"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Prs1::_0x6
    }
    #[doc = "PCLKB/27"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Prs1::_0x7
    }
    #[doc = "PCLKB/28"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Prs1::_0x8
    }
    #[doc = "PCLKB/29"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Prs1::_0x9
    }
    #[doc = "PCLKB/210"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Prs1::_0xA
    }
    #[doc = "PCLKB/211"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Prs1::_0xB
    }
    #[doc = "PCLKB/212"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Prs1::_0xC
    }
    #[doc = "PCLKB/213"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Prs1::_0xD
    }
    #[doc = "PCLKB/214"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Prs1::_0xE
    }
    #[doc = "PCLKB/215"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Prs1::_0xF
    }
}
#[doc = "Field `PRS1` writer - Selection of operation clock (CKm1)"]
pub type Prs1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Prs1, crate::Safe>;
impl<'a, REG> Prs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x0)
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x1)
    }
    #[doc = "PCLKB/22"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x2)
    }
    #[doc = "PCLKB/23"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x3)
    }
    #[doc = "PCLKB/24"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x4)
    }
    #[doc = "PCLKB/25"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x5)
    }
    #[doc = "PCLKB/26"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x6)
    }
    #[doc = "PCLKB/27"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x7)
    }
    #[doc = "PCLKB/28"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x8)
    }
    #[doc = "PCLKB/29"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0x9)
    }
    #[doc = "PCLKB/210"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xA)
    }
    #[doc = "PCLKB/211"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xB)
    }
    #[doc = "PCLKB/212"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xC)
    }
    #[doc = "PCLKB/213"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xD)
    }
    #[doc = "PCLKB/214"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xE)
    }
    #[doc = "PCLKB/215"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Prs1::_0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selection of operation clock (CKm0)"]
    #[inline(always)]
    pub fn prs0(&self) -> Prs0R {
        Prs0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selection of operation clock (CKm1)"]
    #[inline(always)]
    pub fn prs1(&self) -> Prs1R {
        Prs1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of operation clock (CKm0)"]
    #[inline(always)]
    pub fn prs0(&mut self) -> Prs0W<Sps0Spec> {
        Prs0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selection of operation clock (CKm1)"]
    #[inline(always)]
    pub fn prs1(&mut self) -> Prs1W<Sps0Spec> {
        Prs1W::new(self, 4)
    }
}
#[doc = "Serial Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sps0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sps0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sps0Spec;
impl crate::RegisterSpec for Sps0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sps0::R`](R) reader structure"]
impl crate::Readable for Sps0Spec {}
#[doc = "`write(|w| ..)` method takes [`sps0::W`](W) writer structure"]
impl crate::Writable for Sps0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPS0 to value 0"]
impl crate::Resettable for Sps0Spec {
    const RESET_VALUE: u16 = 0;
}
