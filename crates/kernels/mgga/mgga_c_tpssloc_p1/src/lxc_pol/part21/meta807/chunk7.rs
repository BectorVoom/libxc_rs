//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2817/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817<F: Float>(t1484: F, t2745: F, t17109: F, t2752: F, t13471: F, t16662: F, t17116: F, t1877: F, t2522: F, t262: F, t41254: F, t41258: F, t41262: F, t4307: F, t4314: F, t58983: F, t58985: F, t58986: F, t58987: F, t776: F, t868: F) -> F {
    let t59580 = t1484 * t2745;
    let t59584 = t17109 * t2752;
    let t59591 = F::cast_from(12.0_f64) * t16662 * t262 * t4314 * t776 - F::cast_from(2.0_f64) * t13471 * t1877 * t4307 - t17116 * t1877 * t2745 - F::cast_from(2.0_f64) * t1877 * t59584 * t868 - F::cast_from(6.0_f64) * t2522 * t4307 * t59580 + t41254 - t41258 - t41262 - t58983 + t58985 - t58986 - t58987;
    t59591
}
