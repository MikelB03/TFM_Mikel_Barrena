#[doc = "Register `PDR` reader"]
pub type R = crate::R<PdrSpec>;
#[doc = "Register `PDR` writer"]
pub type W = crate::W<PdrSpec>;
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr00 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr00> for bool {
    #[inline(always)]
    fn from(variant: Pdr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR00` reader - Pin and Pjn Direction"]
pub type Pdr00R = crate::BitReader<Pdr00>;
impl Pdr00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr00 {
        match self.bits {
            false => Pdr00::_0,
            true => Pdr00::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr00::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr00::_1
    }
}
#[doc = "Field `PDR00` writer - Pin and Pjn Direction"]
pub type Pdr00W<'a, REG> = crate::BitWriter<'a, REG, Pdr00>;
impl<'a, REG> Pdr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr00::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr00::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr01 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr01> for bool {
    #[inline(always)]
    fn from(variant: Pdr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR01` reader - Pin and Pjn Direction"]
pub type Pdr01R = crate::BitReader<Pdr01>;
impl Pdr01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr01 {
        match self.bits {
            false => Pdr01::_0,
            true => Pdr01::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr01::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr01::_1
    }
}
#[doc = "Field `PDR01` writer - Pin and Pjn Direction"]
pub type Pdr01W<'a, REG> = crate::BitWriter<'a, REG, Pdr01>;
impl<'a, REG> Pdr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr01::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr01::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr02 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr02> for bool {
    #[inline(always)]
    fn from(variant: Pdr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR02` reader - Pin and Pjn Direction"]
pub type Pdr02R = crate::BitReader<Pdr02>;
impl Pdr02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr02 {
        match self.bits {
            false => Pdr02::_0,
            true => Pdr02::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr02::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr02::_1
    }
}
#[doc = "Field `PDR02` writer - Pin and Pjn Direction"]
pub type Pdr02W<'a, REG> = crate::BitWriter<'a, REG, Pdr02>;
impl<'a, REG> Pdr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr02::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr02::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr03 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr03> for bool {
    #[inline(always)]
    fn from(variant: Pdr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR03` reader - Pin and Pjn Direction"]
pub type Pdr03R = crate::BitReader<Pdr03>;
impl Pdr03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr03 {
        match self.bits {
            false => Pdr03::_0,
            true => Pdr03::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr03::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr03::_1
    }
}
#[doc = "Field `PDR03` writer - Pin and Pjn Direction"]
pub type Pdr03W<'a, REG> = crate::BitWriter<'a, REG, Pdr03>;
impl<'a, REG> Pdr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr03::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr03::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr04 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr04> for bool {
    #[inline(always)]
    fn from(variant: Pdr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR04` reader - Pin and Pjn Direction"]
pub type Pdr04R = crate::BitReader<Pdr04>;
impl Pdr04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr04 {
        match self.bits {
            false => Pdr04::_0,
            true => Pdr04::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr04::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr04::_1
    }
}
#[doc = "Field `PDR04` writer - Pin and Pjn Direction"]
pub type Pdr04W<'a, REG> = crate::BitWriter<'a, REG, Pdr04>;
impl<'a, REG> Pdr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr04::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr04::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr05 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr05> for bool {
    #[inline(always)]
    fn from(variant: Pdr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR05` reader - Pin and Pjn Direction"]
pub type Pdr05R = crate::BitReader<Pdr05>;
impl Pdr05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr05 {
        match self.bits {
            false => Pdr05::_0,
            true => Pdr05::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr05::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr05::_1
    }
}
#[doc = "Field `PDR05` writer - Pin and Pjn Direction"]
pub type Pdr05W<'a, REG> = crate::BitWriter<'a, REG, Pdr05>;
impl<'a, REG> Pdr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr05::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr05::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr06 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr06> for bool {
    #[inline(always)]
    fn from(variant: Pdr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR06` reader - Pin and Pjn Direction"]
pub type Pdr06R = crate::BitReader<Pdr06>;
impl Pdr06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr06 {
        match self.bits {
            false => Pdr06::_0,
            true => Pdr06::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr06::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr06::_1
    }
}
#[doc = "Field `PDR06` writer - Pin and Pjn Direction"]
pub type Pdr06W<'a, REG> = crate::BitWriter<'a, REG, Pdr06>;
impl<'a, REG> Pdr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr06::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr06::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr07 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr07> for bool {
    #[inline(always)]
    fn from(variant: Pdr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR07` reader - Pin and Pjn Direction"]
pub type Pdr07R = crate::BitReader<Pdr07>;
impl Pdr07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr07 {
        match self.bits {
            false => Pdr07::_0,
            true => Pdr07::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr07::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr07::_1
    }
}
#[doc = "Field `PDR07` writer - Pin and Pjn Direction"]
pub type Pdr07W<'a, REG> = crate::BitWriter<'a, REG, Pdr07>;
impl<'a, REG> Pdr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr07::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr07::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr08 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr08> for bool {
    #[inline(always)]
    fn from(variant: Pdr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR08` reader - Pin and Pjn Direction"]
pub type Pdr08R = crate::BitReader<Pdr08>;
impl Pdr08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr08 {
        match self.bits {
            false => Pdr08::_0,
            true => Pdr08::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr08::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr08::_1
    }
}
#[doc = "Field `PDR08` writer - Pin and Pjn Direction"]
pub type Pdr08W<'a, REG> = crate::BitWriter<'a, REG, Pdr08>;
impl<'a, REG> Pdr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr08::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr08::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr09 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr09> for bool {
    #[inline(always)]
    fn from(variant: Pdr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR09` reader - Pin and Pjn Direction"]
pub type Pdr09R = crate::BitReader<Pdr09>;
impl Pdr09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr09 {
        match self.bits {
            false => Pdr09::_0,
            true => Pdr09::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr09::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr09::_1
    }
}
#[doc = "Field `PDR09` writer - Pin and Pjn Direction"]
pub type Pdr09W<'a, REG> = crate::BitWriter<'a, REG, Pdr09>;
impl<'a, REG> Pdr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr09::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr09::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr10 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr10> for bool {
    #[inline(always)]
    fn from(variant: Pdr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR10` reader - Pin and Pjn Direction"]
pub type Pdr10R = crate::BitReader<Pdr10>;
impl Pdr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr10 {
        match self.bits {
            false => Pdr10::_0,
            true => Pdr10::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr10::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr10::_1
    }
}
#[doc = "Field `PDR10` writer - Pin and Pjn Direction"]
pub type Pdr10W<'a, REG> = crate::BitWriter<'a, REG, Pdr10>;
impl<'a, REG> Pdr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr10::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr10::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr11 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr11> for bool {
    #[inline(always)]
    fn from(variant: Pdr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR11` reader - Pin and Pjn Direction"]
pub type Pdr11R = crate::BitReader<Pdr11>;
impl Pdr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr11 {
        match self.bits {
            false => Pdr11::_0,
            true => Pdr11::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr11::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr11::_1
    }
}
#[doc = "Field `PDR11` writer - Pin and Pjn Direction"]
pub type Pdr11W<'a, REG> = crate::BitWriter<'a, REG, Pdr11>;
impl<'a, REG> Pdr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr11::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr11::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr12 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr12> for bool {
    #[inline(always)]
    fn from(variant: Pdr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR12` reader - Pin and Pjn Direction"]
pub type Pdr12R = crate::BitReader<Pdr12>;
impl Pdr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr12 {
        match self.bits {
            false => Pdr12::_0,
            true => Pdr12::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr12::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr12::_1
    }
}
#[doc = "Field `PDR12` writer - Pin and Pjn Direction"]
pub type Pdr12W<'a, REG> = crate::BitWriter<'a, REG, Pdr12>;
impl<'a, REG> Pdr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr12::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr12::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr13 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr13> for bool {
    #[inline(always)]
    fn from(variant: Pdr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR13` reader - Pin and Pjn Direction"]
pub type Pdr13R = crate::BitReader<Pdr13>;
impl Pdr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr13 {
        match self.bits {
            false => Pdr13::_0,
            true => Pdr13::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr13::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr13::_1
    }
}
#[doc = "Field `PDR13` writer - Pin and Pjn Direction"]
pub type Pdr13W<'a, REG> = crate::BitWriter<'a, REG, Pdr13>;
impl<'a, REG> Pdr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr13::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr13::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr14 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr14> for bool {
    #[inline(always)]
    fn from(variant: Pdr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR14` reader - Pin and Pjn Direction"]
pub type Pdr14R = crate::BitReader<Pdr14>;
impl Pdr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr14 {
        match self.bits {
            false => Pdr14::_0,
            true => Pdr14::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr14::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr14::_1
    }
}
#[doc = "Field `PDR14` writer - Pin and Pjn Direction"]
pub type Pdr14W<'a, REG> = crate::BitWriter<'a, REG, Pdr14>;
impl<'a, REG> Pdr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr14::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr14::_1)
    }
}
#[doc = "Pin and Pjn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr15 {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr15> for bool {
    #[inline(always)]
    fn from(variant: Pdr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR15` reader - Pin and Pjn Direction"]
pub type Pdr15R = crate::BitReader<Pdr15>;
impl Pdr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr15 {
        match self.bits {
            false => Pdr15::_0,
            true => Pdr15::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr15::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr15::_1
    }
}
#[doc = "Field `PDR15` writer - Pin and Pjn Direction"]
pub type Pdr15W<'a, REG> = crate::BitWriter<'a, REG, Pdr15>;
impl<'a, REG> Pdr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr15::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr15::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr00(&self) -> Pdr00R {
        Pdr00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr01(&self) -> Pdr01R {
        Pdr01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr02(&self) -> Pdr02R {
        Pdr02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr03(&self) -> Pdr03R {
        Pdr03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr04(&self) -> Pdr04R {
        Pdr04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr05(&self) -> Pdr05R {
        Pdr05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr06(&self) -> Pdr06R {
        Pdr06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr07(&self) -> Pdr07R {
        Pdr07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr08(&self) -> Pdr08R {
        Pdr08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr09(&self) -> Pdr09R {
        Pdr09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr10(&self) -> Pdr10R {
        Pdr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr11(&self) -> Pdr11R {
        Pdr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr12(&self) -> Pdr12R {
        Pdr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr13(&self) -> Pdr13R {
        Pdr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr14(&self) -> Pdr14R {
        Pdr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr15(&self) -> Pdr15R {
        Pdr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr00(&mut self) -> Pdr00W<PdrSpec> {
        Pdr00W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr01(&mut self) -> Pdr01W<PdrSpec> {
        Pdr01W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr02(&mut self) -> Pdr02W<PdrSpec> {
        Pdr02W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr03(&mut self) -> Pdr03W<PdrSpec> {
        Pdr03W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr04(&mut self) -> Pdr04W<PdrSpec> {
        Pdr04W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr05(&mut self) -> Pdr05W<PdrSpec> {
        Pdr05W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr06(&mut self) -> Pdr06W<PdrSpec> {
        Pdr06W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr07(&mut self) -> Pdr07W<PdrSpec> {
        Pdr07W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr08(&mut self) -> Pdr08W<PdrSpec> {
        Pdr08W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr09(&mut self) -> Pdr09W<PdrSpec> {
        Pdr09W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr10(&mut self) -> Pdr10W<PdrSpec> {
        Pdr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr11(&mut self) -> Pdr11W<PdrSpec> {
        Pdr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr12(&mut self) -> Pdr12W<PdrSpec> {
        Pdr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr13(&mut self) -> Pdr13W<PdrSpec> {
        Pdr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr14(&mut self) -> Pdr14W<PdrSpec> {
        Pdr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin and Pjn Direction"]
    #[inline(always)]
    pub fn pdr15(&mut self) -> Pdr15W<PdrSpec> {
        Pdr15W::new(self, 15)
    }
}
#[doc = "Port Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdrSpec;
impl crate::RegisterSpec for PdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdr::R`](R) reader structure"]
impl crate::Readable for PdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pdr::W`](W) writer structure"]
impl crate::Writable for PdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PdrSpec {
    const RESET_VALUE: u16 = 0;
}
