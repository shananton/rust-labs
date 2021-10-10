// MIT License
//
// Copyright (c) 2021 Exercism
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Все упоминания `PhantomData` в этом файле можно убрать, они только для того,
// чтобы текущий код компилировался

/// Правило для FizzBuzz: с помощью заданного предиката мы проверяем, должен ли
/// текущий элемент T быть заменен на слово? Если да, то на какое?
pub struct Matcher<T: Copy + ToString + PartialEq> {
    predicate: Box<dyn Fn(T) -> bool>,
    substitute_with: String,
}

impl<T: Copy + ToString + PartialEq> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: ToString>(predicate: F, substitute_with: S) -> Self {
        Self {
            predicate: Box::new(predicate),
            substitute_with: substitute_with.to_string(),
        }
    }
}

/// Набор правил Matcher, которые можно применить к итератору.
///
/// Более идиоматично использовать метод `.map()` для модификации итератора
/// вместо метода `Fizzy::apply()`, который этот итератор поглощает.
///
/// Зато можно попрактиковаться с более простым интерфейсом `apply`.
pub struct Fizzy<T: Copy + ToString + PartialEq> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Copy + ToString + PartialEq> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: Vec::new() }
    }

    // можете использовать `mut self` в сигнатуре, если вам нравится
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// Применяет набор Matchers к каждому элементу итератора
    pub fn apply<I: Iterator<Item=T>>(self, iter: I) -> impl Iterator<Item=String> {
        iter.map(move |elem| {
            let replacements = self.matchers.iter().flat_map(|m| {
                if (*m.predicate)(elem) { Some(m.substitute_with.clone()) } else { None }
            }).collect::<Vec<_>>();
            if replacements.is_empty() {
                elem.to_string()
            } else {
                replacements.join("")
            }
        })
    }
}

/// Вспомогательная функция: возвращает `Fizzy` со стандартными правилами FizzBuzz
pub fn fizz_buzz<T: Copy + ToString + PartialEq + From<u8> + std::ops::Rem<T, Output=T>>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|x: T| x % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|x: T| x % 5.into() == 0.into(), "buzz"))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        () => {
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16",
            ]
        };
    }

    #[test]
    fn test_simple() {
        let got = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u8() {
        let got = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u64() {
        let got = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let expect = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        let got = fizz_buzz::<i32>()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));
        let got = fizzer.apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_f64() {
        // a tiny bit more complicated because range isn't natively implemented on floats
        let got = fizz_buzz::<f64>()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_minimal_generic_bounds() {
        use std::fmt;
        use std::ops::{Add, Rem};

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Fizzable(u8);

        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }

        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(ref n) = self;
                write!(f, "{}", n)
            }
        }

        impl Add for Fizzable {
            type Output = Fizzable;
            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 + n2)
            }
        }

        impl Rem for Fizzable {
            type Output = Fizzable;
            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 % n2)
            }
        }

        let got = fizz_buzz::<Fizzable>()
            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }
}
