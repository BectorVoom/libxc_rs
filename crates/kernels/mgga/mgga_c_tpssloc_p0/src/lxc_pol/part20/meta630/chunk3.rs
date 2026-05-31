//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2287/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2287<F: Float>(t46528: F, t816: F, t4159: F, t9541: F, t120: F, t12971: F, t13173: F, t13177: F, t13193: F, t13198: F, t13302: F, t2618: F, t2623: F, t2643: F, t2645: F, t2681: F, t41355: F, t41363: F, t41365: F, t41373: F, t41386: F, t47215: F, t817: F, t819: F, t820: F, t829: F, t831: F, t9642: F) -> F {
    let t47220 = t46528 * t816;
    let t47230 = t9541 * t4159;
    let t47231 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t47230;
    let t47239 = F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t2623 * t13193 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2623 * t13198 - t2618 * t13173 / F::cast_from(1024.0_f64) - t817 * t819 * t820 * t47215 / F::cast_from(3072.0_f64) - t47220 * t831 / F::cast_from(1024.0_f64) - t13177 * t2681 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t41355 + F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t41363 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t41365 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t41373 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t41386 - t47231 + t9642 * t13302 / F::cast_from(128.0_f64) + t2643 * t2645 * t120 * t12971 * t829 / F::cast_from(256.0_f64);
    t47239
}
