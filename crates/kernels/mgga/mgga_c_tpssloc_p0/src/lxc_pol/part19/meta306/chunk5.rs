//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1098/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098<F: Float>(t41: F, t42: F, t53: F, t54: F, t1028: F, t36: F, t9576: F, t2244: F, t2250: F, t2262: F, t2267: F, t2268: F, t2271: F, t2274: F, t39: F, t39097: F, t39103: F, t39110: F, t43: F, t44: F, t51: F, t55: F, t615: F, t618: F, t9258: F, t9277: F, t9287: F, t9289: F, t9292: F, t9293: F, t9296: F, t9300: F, t9304: F, sigma0: F) -> (F, F) {
    let t39157 = t41 * t41;
    let t39159 = F::cast_from(1.0_f64) / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = F::cast_from(1.0_f64) / t54 / t39166;
    let t39176 = F::cast_from(1.0_f64) / t36 / t1028;
    let t39177 = sigma0 * t39176;
    let t39210 = F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t9576;
    let t39213 = F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t39 * t39159 * t39097 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t43 * t39110 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t51 * t39168 * t39097 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t55 * t39110 + F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t39177 * t44 - F::cast_from(12320.0_f64) / F::cast_from(81.0_f64) * t9277 * t618 + F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t2262 * t2271 + F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t2262 * t2268 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t615 * t9289 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t615 * t9296 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t9287 * t2244 * t2250 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t2267 * t39103 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t39 * t9292 * t9258 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t9300 * t2244 * t2250 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t2274 * t39103 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t51 * t9304 * t9258 - t39210 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t615 * t9293;
    (t39177, t39213)
}
