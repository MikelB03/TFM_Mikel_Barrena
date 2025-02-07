#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - ICU Interrupt 0"]
    IEL0 = 0,
    #[doc = "1 - ICU Interrupt 1"]
    IEL1 = 1,
    #[doc = "2 - ICU Interrupt 2"]
    IEL2 = 2,
    #[doc = "3 - ICU Interrupt 3"]
    IEL3 = 3,
    #[doc = "4 - ICU Interrupt 4"]
    IEL4 = 4,
    #[doc = "5 - ICU Interrupt 5"]
    IEL5 = 5,
    #[doc = "6 - ICU Interrupt 6"]
    IEL6 = 6,
    #[doc = "7 - ICU Interrupt 7"]
    IEL7 = 7,
    #[doc = "8 - ICU Interrupt 8"]
    IEL8 = 8,
    #[doc = "9 - ICU Interrupt 9"]
    IEL9 = 9,
    #[doc = "10 - ICU Interrupt 10"]
    IEL10 = 10,
    #[doc = "11 - ICU Interrupt 11"]
    IEL11 = 11,
    #[doc = "12 - ICU Interrupt 12"]
    IEL12 = 12,
    #[doc = "13 - ICU Interrupt 13"]
    IEL13 = 13,
    #[doc = "14 - ICU Interrupt 14"]
    IEL14 = 14,
    #[doc = "15 - ICU Interrupt 15"]
    IEL15 = 15,
    #[doc = "16 - ICU Interrupt 16"]
    IEL16 = 16,
    #[doc = "17 - ICU Interrupt 17"]
    IEL17 = 17,
    #[doc = "18 - ICU Interrupt 18"]
    IEL18 = 18,
    #[doc = "19 - ICU Interrupt 19"]
    IEL19 = 19,
    #[doc = "20 - ICU Interrupt 20"]
    IEL20 = 20,
    #[doc = "21 - ICU Interrupt 21"]
    IEL21 = 21,
    #[doc = "22 - ICU Interrupt 22"]
    IEL22 = 22,
    #[doc = "23 - ICU Interrupt 23"]
    IEL23 = 23,
    #[doc = "24 - ICU Interrupt 24"]
    IEL24 = 24,
    #[doc = "25 - ICU Interrupt 25"]
    IEL25 = 25,
    #[doc = "26 - ICU Interrupt 26"]
    IEL26 = 26,
    #[doc = "27 - ICU Interrupt 27"]
    IEL27 = 27,
    #[doc = "28 - ICU Interrupt 28"]
    IEL28 = 28,
    #[doc = "29 - ICU Interrupt 29"]
    IEL29 = 29,
    #[doc = "30 - ICU Interrupt 30"]
    IEL30 = 30,
    #[doc = "31 - ICU Interrupt 31"]
    IEL31 = 31,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::IEL0),
            1 => Ok(Interrupt::IEL1),
            2 => Ok(Interrupt::IEL2),
            3 => Ok(Interrupt::IEL3),
            4 => Ok(Interrupt::IEL4),
            5 => Ok(Interrupt::IEL5),
            6 => Ok(Interrupt::IEL6),
            7 => Ok(Interrupt::IEL7),
            8 => Ok(Interrupt::IEL8),
            9 => Ok(Interrupt::IEL9),
            10 => Ok(Interrupt::IEL10),
            11 => Ok(Interrupt::IEL11),
            12 => Ok(Interrupt::IEL12),
            13 => Ok(Interrupt::IEL13),
            14 => Ok(Interrupt::IEL14),
            15 => Ok(Interrupt::IEL15),
            16 => Ok(Interrupt::IEL16),
            17 => Ok(Interrupt::IEL17),
            18 => Ok(Interrupt::IEL18),
            19 => Ok(Interrupt::IEL19),
            20 => Ok(Interrupt::IEL20),
            21 => Ok(Interrupt::IEL21),
            22 => Ok(Interrupt::IEL22),
            23 => Ok(Interrupt::IEL23),
            24 => Ok(Interrupt::IEL24),
            25 => Ok(Interrupt::IEL25),
            26 => Ok(Interrupt::IEL26),
            27 => Ok(Interrupt::IEL27),
            28 => Ok(Interrupt::IEL28),
            29 => Ok(Interrupt::IEL29),
            30 => Ok(Interrupt::IEL30),
            31 => Ok(Interrupt::IEL31),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
