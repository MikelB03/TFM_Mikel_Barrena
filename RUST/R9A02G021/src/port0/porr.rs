#[doc = "Register `PORR` writer"]
pub type W = crate::W<PorrSpec>;
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr00 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr00> for bool {
    #[inline(always)]
    fn from(variant: Porr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR00` writer - Pin and Pjn Output Reset"]
pub type Porr00W<'a, REG> = crate::BitWriter<'a, REG, Porr00>;
impl<'a, REG> Porr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr01 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr01> for bool {
    #[inline(always)]
    fn from(variant: Porr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR01` writer - Pin and Pjn Output Reset"]
pub type Porr01W<'a, REG> = crate::BitWriter<'a, REG, Porr01>;
impl<'a, REG> Porr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr02 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr02> for bool {
    #[inline(always)]
    fn from(variant: Porr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR02` writer - Pin and Pjn Output Reset"]
pub type Porr02W<'a, REG> = crate::BitWriter<'a, REG, Porr02>;
impl<'a, REG> Porr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr03 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr03> for bool {
    #[inline(always)]
    fn from(variant: Porr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR03` writer - Pin and Pjn Output Reset"]
pub type Porr03W<'a, REG> = crate::BitWriter<'a, REG, Porr03>;
impl<'a, REG> Porr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr04 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr04> for bool {
    #[inline(always)]
    fn from(variant: Porr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR04` writer - Pin and Pjn Output Reset"]
pub type Porr04W<'a, REG> = crate::BitWriter<'a, REG, Porr04>;
impl<'a, REG> Porr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr05 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr05> for bool {
    #[inline(always)]
    fn from(variant: Porr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR05` writer - Pin and Pjn Output Reset"]
pub type Porr05W<'a, REG> = crate::BitWriter<'a, REG, Porr05>;
impl<'a, REG> Porr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr06 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr06> for bool {
    #[inline(always)]
    fn from(variant: Porr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR06` writer - Pin and Pjn Output Reset"]
pub type Porr06W<'a, REG> = crate::BitWriter<'a, REG, Porr06>;
impl<'a, REG> Porr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr07 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr07> for bool {
    #[inline(always)]
    fn from(variant: Porr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR07` writer - Pin and Pjn Output Reset"]
pub type Porr07W<'a, REG> = crate::BitWriter<'a, REG, Porr07>;
impl<'a, REG> Porr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr08 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr08> for bool {
    #[inline(always)]
    fn from(variant: Porr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR08` writer - Pin and Pjn Output Reset"]
pub type Porr08W<'a, REG> = crate::BitWriter<'a, REG, Porr08>;
impl<'a, REG> Porr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr09 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr09> for bool {
    #[inline(always)]
    fn from(variant: Porr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR09` writer - Pin and Pjn Output Reset"]
pub type Porr09W<'a, REG> = crate::BitWriter<'a, REG, Porr09>;
impl<'a, REG> Porr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr10 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr10> for bool {
    #[inline(always)]
    fn from(variant: Porr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR10` writer - Pin and Pjn Output Reset"]
pub type Porr10W<'a, REG> = crate::BitWriter<'a, REG, Porr10>;
impl<'a, REG> Porr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr11 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr11> for bool {
    #[inline(always)]
    fn from(variant: Porr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR11` writer - Pin and Pjn Output Reset"]
pub type Porr11W<'a, REG> = crate::BitWriter<'a, REG, Porr11>;
impl<'a, REG> Porr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr12 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr12> for bool {
    #[inline(always)]
    fn from(variant: Porr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR12` writer - Pin and Pjn Output Reset"]
pub type Porr12W<'a, REG> = crate::BitWriter<'a, REG, Porr12>;
impl<'a, REG> Porr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr13 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr13> for bool {
    #[inline(always)]
    fn from(variant: Porr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR13` writer - Pin and Pjn Output Reset"]
pub type Porr13W<'a, REG> = crate::BitWriter<'a, REG, Porr13>;
impl<'a, REG> Porr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr14 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr14> for bool {
    #[inline(always)]
    fn from(variant: Porr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR14` writer - Pin and Pjn Output Reset"]
pub type Porr14W<'a, REG> = crate::BitWriter<'a, REG, Porr14>;
impl<'a, REG> Porr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr15 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr15> for bool {
    #[inline(always)]
    fn from(variant: Porr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR15` writer - Pin and Pjn Output Reset"]
pub type Porr15W<'a, REG> = crate::BitWriter<'a, REG, Porr15>;
impl<'a, REG> Porr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr00(&mut self) -> Porr00W<PorrSpec> {
        Porr00W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr01(&mut self) -> Porr01W<PorrSpec> {
        Porr01W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr02(&mut self) -> Porr02W<PorrSpec> {
        Porr02W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr03(&mut self) -> Porr03W<PorrSpec> {
        Porr03W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr04(&mut self) -> Porr04W<PorrSpec> {
        Porr04W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr05(&mut self) -> Porr05W<PorrSpec> {
        Porr05W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr06(&mut self) -> Porr06W<PorrSpec> {
        Porr06W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr07(&mut self) -> Porr07W<PorrSpec> {
        Porr07W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr08(&mut self) -> Porr08W<PorrSpec> {
        Porr08W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr09(&mut self) -> Porr09W<PorrSpec> {
        Porr09W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr10(&mut self) -> Porr10W<PorrSpec> {
        Porr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr11(&mut self) -> Porr11W<PorrSpec> {
        Porr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr12(&mut self) -> Porr12W<PorrSpec> {
        Porr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr13(&mut self) -> Porr13W<PorrSpec> {
        Porr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr14(&mut self) -> Porr14W<PorrSpec> {
        Porr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr15(&mut self) -> Porr15W<PorrSpec> {
        Porr15W::new(self, 15)
    }
}
#[doc = "Port Control Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PorrSpec;
impl crate::RegisterSpec for PorrSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`porr::W`](W) writer structure"]
impl crate::Writable for PorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PORR to value 0"]
impl crate::Resettable for PorrSpec {
    const RESET_VALUE: u16 = 0;
}
