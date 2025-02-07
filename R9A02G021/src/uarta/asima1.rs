#[doc = "Register `ASIMA%s1` reader"]
pub type R = crate::R<Asima1Spec>;
#[doc = "Register `ASIMA%s1` writer"]
pub type W = crate::W<Asima1Spec>;
#[doc = "Transmission and reception level setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alv {
    #[doc = "0: Positive logic (wait state = high level, start bit = low level, stop bit = high level)"]
    _0 = 0,
    #[doc = "1: Negative logic (wait state = low level, start bit = high level, stop bit = low level)"]
    _1 = 1,
}
impl From<Alv> for bool {
    #[inline(always)]
    fn from(variant: Alv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALV` reader - Transmission and reception level setting"]
pub type AlvR = crate::BitReader<Alv>;
impl AlvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alv {
        match self.bits {
            false => Alv::_0,
            true => Alv::_1,
        }
    }
    #[doc = "Positive logic (wait state = high level, start bit = low level, stop bit = high level)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alv::_0
    }
    #[doc = "Negative logic (wait state = low level, start bit = high level, stop bit = low level)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alv::_1
    }
}
#[doc = "Field `ALV` writer - Transmission and reception level setting"]
pub type AlvW<'a, REG> = crate::BitWriter<'a, REG, Alv>;
impl<'a, REG> AlvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive logic (wait state = high level, start bit = low level, stop bit = high level)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alv::_0)
    }
    #[doc = "Negative logic (wait state = low level, start bit = high level, stop bit = low level)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alv::_1)
    }
}
#[doc = "Transmission and reception order setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: MSB-first"]
    _0 = 0,
    #[doc = "1: LSB-first"]
    _1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transmission and reception order setting"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::_0,
            true => Dir::_1,
        }
    }
    #[doc = "MSB-first"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dir::_0
    }
    #[doc = "LSB-first"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dir::_1
    }
}
#[doc = "Field `DIR` writer - Transmission and reception order setting"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB-first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_0)
    }
    #[doc = "LSB-first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_1)
    }
}
#[doc = "Transmission stop bit length setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sl {
    #[doc = "0: Stop bit length = 1 bit"]
    _0 = 0,
    #[doc = "1: Stop bit length = 2 bits"]
    _1 = 1,
}
impl From<Sl> for bool {
    #[inline(always)]
    fn from(variant: Sl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SL` reader - Transmission stop bit length setting"]
pub type SlR = crate::BitReader<Sl>;
impl SlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sl {
        match self.bits {
            false => Sl::_0,
            true => Sl::_1,
        }
    }
    #[doc = "Stop bit length = 1 bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sl::_0
    }
    #[doc = "Stop bit length = 2 bits"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sl::_1
    }
}
#[doc = "Field `SL` writer - Transmission stop bit length setting"]
pub type SlW<'a, REG> = crate::BitWriter<'a, REG, Sl>;
impl<'a, REG> SlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop bit length = 1 bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sl::_0)
    }
    #[doc = "Stop bit length = 2 bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sl::_1)
    }
}
#[doc = "Transmission and reception character length setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cl {
    #[doc = "0: Character length of data = 5 bits"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Character length of data = 7 bits"]
    _10 = 2,
    #[doc = "3: Character length of data = 8 bits"]
    _11 = 3,
}
impl From<Cl> for u8 {
    #[inline(always)]
    fn from(variant: Cl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cl {
    type Ux = u8;
}
impl crate::IsEnum for Cl {}
#[doc = "Field `CL` reader - Transmission and reception character length setting"]
pub type ClR = crate::FieldReader<Cl>;
impl ClR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cl {
        match self.bits {
            0 => Cl::_00,
            1 => Cl::_01,
            2 => Cl::_10,
            3 => Cl::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Character length of data = 5 bits"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cl::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cl::_01
    }
    #[doc = "Character length of data = 7 bits"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cl::_10
    }
    #[doc = "Character length of data = 8 bits"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cl::_11
    }
}
#[doc = "Field `CL` writer - Transmission and reception character length setting"]
pub type ClW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cl, crate::Safe>;
impl<'a, REG> ClW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length of data = 5 bits"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cl::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cl::_01)
    }
    #[doc = "Character length of data = 7 bits"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cl::_10)
    }
    #[doc = "Character length of data = 8 bits"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cl::_11)
    }
}
#[doc = "Transmission and reception parity bit setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: Transmission: No parity bit is output. Reception: Data is received without parity."]
    _00 = 0,
    #[doc = "1: Transmission: 0 parity is output. Reception: Data is received with 0 parity."]
    _01 = 1,
    #[doc = "2: Transmission: Odd parity is output. Reception: Check is made for odd parity."]
    _10 = 2,
    #[doc = "3: Transmission: Even parity is output. Reception: Check is made for even parity."]
    _11 = 3,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Transmission and reception parity bit setting"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::_00,
            1 => Ps::_01,
            2 => Ps::_10,
            3 => Ps::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Transmission: No parity bit is output. Reception: Data is received without parity."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ps::_00
    }
    #[doc = "Transmission: 0 parity is output. Reception: Data is received with 0 parity."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ps::_01
    }
    #[doc = "Transmission: Odd parity is output. Reception: Check is made for odd parity."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ps::_10
    }
    #[doc = "Transmission: Even parity is output. Reception: Check is made for even parity."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ps::_11
    }
}
#[doc = "Field `PS` writer - Transmission and reception parity bit setting"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission: No parity bit is output. Reception: Data is received without parity."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_00)
    }
    #[doc = "Transmission: 0 parity is output. Reception: Data is received with 0 parity."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_01)
    }
    #[doc = "Transmission: Odd parity is output. Reception: Check is made for odd parity."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_10)
    }
    #[doc = "Transmission: Even parity is output. Reception: Check is made for even parity."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission and reception level setting"]
    #[inline(always)]
    pub fn alv(&self) -> AlvR {
        AlvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission and reception order setting"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission stop bit length setting"]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Transmission and reception character length setting"]
    #[inline(always)]
    pub fn cl(&self) -> ClR {
        ClR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:6 - Transmission and reception parity bit setting"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission and reception level setting"]
    #[inline(always)]
    pub fn alv(&mut self) -> AlvW<Asima1Spec> {
        AlvW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmission and reception order setting"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<Asima1Spec> {
        DirW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission stop bit length setting"]
    #[inline(always)]
    pub fn sl(&mut self) -> SlW<Asima1Spec> {
        SlW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Transmission and reception character length setting"]
    #[inline(always)]
    pub fn cl(&mut self) -> ClW<Asima1Spec> {
        ClW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Transmission and reception parity bit setting"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<Asima1Spec> {
        PsW::new(self, 5)
    }
}
#[doc = "Operation Mode Setting Register n1\n\nYou can [`read`](crate::Reg::read) this register and get [`asima1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asima1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Asima1Spec;
impl crate::RegisterSpec for Asima1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asima1::R`](R) reader structure"]
impl crate::Readable for Asima1Spec {}
#[doc = "`write(|w| ..)` method takes [`asima1::W`](W) writer structure"]
impl crate::Writable for Asima1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ASIMA%s1 to value 0x1a"]
impl crate::Resettable for Asima1Spec {
    const RESET_VALUE: u8 = 0x1a;
}
