use crate::colors::{Colors, color_brackets};
use std::fmt;
use std::fmt::Write;
#[cfg(feature = "float_rand")]
use ucalc_lib::Rand;
use ucalc_lib::{Functions, Number, ParseReturn, Tokens, Variables, get_help};
use ucalc_numbers::FloatTrait;
#[derive(Clone, Copy)]
pub struct Options {
    pub rpn: bool,
    pub perf: bool,
    pub base_input: u8,
    pub base_output: u8,
    #[cfg(feature = "cli")]
    pub benchmark: usize,
    #[cfg(feature = "cli")]
    pub benchmark_simplify: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            rpn: false,
            perf: false,
            base_input: 10,
            base_output: 10,
            #[cfg(feature = "cli")]
            benchmark: 0,
            #[cfg(feature = "cli")]
            benchmark_simplify: false,
        }
    }
}
#[allow(clippy::too_many_arguments)]
pub fn process_line(
    line: &str,
    vars: &mut Variables,
    funs: &mut Functions,
    options: Options,
    str: &mut String,
    colors: &Colors,
    #[cfg(feature = "float_rand")] rand: &mut Rand,
) -> Result<Option<Number>, fmt::Error> {
    if line.trim_start().starts_with("//") {
        return Ok(None);
    }
    str.clear();
    Ok(match line {
        "" | "exit" | "clear" => None,
        _ if line.starts_with("help") => {
            let arg = line.split_once(' ').map(|(_, a)| a).unwrap_or("");
            write!(str, "{}", color_brackets(get_help(arg), colors))?;
            None
        }
        _ => {
            match tmr_write(
                || {
                    Tokens::parse(
                        line,
                        vars,
                        funs,
                        &[],
                        false,
                        true,
                        options.base_input,
                        options.rpn,
                        #[cfg(feature = "float_rand")]
                        rand,
                    )
                },
                str,
                options.perf,
            ) {
                Ok(ParseReturn::Tokens(tokens)) => {
                    let compute = tmr_write(
                        || {
                            tokens.compute(
                                &[],
                                funs,
                                vars,
                                #[cfg(feature = "float_rand")]
                                rand,
                            )
                        },
                        str,
                        options.perf,
                    );
                    write!(str, "{}", compute.get_closest_fraction(options.base_output))?;
                    write!(str, "{}", compute.to_string_radix(options.base_output))?;
                    Some(compute)
                }
                Ok(ParseReturn::Graph(_, _)) => todo!(),
                Ok(ParseReturn::Var) => None,
                Err(e) => {
                    write!(str, "{e:?}")?;
                    None
                }
            }
        }
    })
}
pub fn to_alt(c: char) -> Option<char> {
    Some(match c {
        'a' => 'α',
        'A' => 'Α',
        'b' => 'β',
        'B' => 'Β',
        'c' => 'ξ',
        'C' => 'Ξ',
        'd' => 'Δ',
        'D' => 'δ',
        'e' => 'ε',
        'E' => 'Ε',
        'f' => 'φ',
        'F' => 'Φ',
        'g' => 'γ',
        'G' => 'Γ',
        'h' => 'η',
        'H' => 'Η',
        'i' => 'ι',
        'I' => 'Ι',
        'k' => 'κ',
        'K' => 'Κ',
        'l' => 'λ',
        'L' => 'Λ',
        'm' => 'μ',
        'M' => 'Μ',
        'n' => 'ν',
        'N' => 'Ν',
        'o' => 'ο',
        'O' => 'Ο',
        'p' => 'π',
        'P' => 'Π',
        'q' => 'θ',
        'Q' => 'Θ',
        'r' => 'ρ',
        'R' => 'Ρ',
        's' => 'σ',
        'S' => 'Σ',
        't' => 'τ',
        'T' => 'Τ',
        'u' => 'υ',
        'U' => 'Υ',
        'w' => 'ω',
        'W' => 'Ω',
        'y' => 'ψ',
        'Y' => 'Ψ',
        'x' => 'χ',
        'X' => 'Χ',
        'z' => 'ζ',
        'Z' => 'Ζ',
        '9' => '⁹',
        '8' => '⁸',
        '7' => '⁷',
        '6' => '⁶',
        '5' => '⁵',
        '4' => '⁴',
        '3' => '³',
        '2' => '²',
        '1' => '¹',
        '0' => '⁰',
        ')' => '₀',
        '!' => '₁',
        '@' => '₂',
        '#' => '₃',
        '$' => '₄',
        '%' => '₅',
        '^' => '₆',
        '&' => '₇',
        '*' => '₈',
        '(' => '₉',
        ';' => '°',
        '-' => '⁻',
        '+' => '⁺',
        '`' => 'ⁱ',
        '=' => '±',
        '_' => '∞',
        _ => return None,
    })
}
fn tmr_write<T, W>(fun: T, str: &mut impl Write, perf: bool) -> W
where
    T: FnOnce() -> W,
{
    if perf {
        let tmr = std::time::Instant::now();
        let ret = fun();
        writeln!(str, "{}", tmr.elapsed().as_nanos()).unwrap();
        ret
    } else {
        fun()
    }
}
