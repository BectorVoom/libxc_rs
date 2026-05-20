//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2943/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943<F: Float>(t10236: F, t17635: F, t13835: F, t13847: F, t2986: F, t13839: F, t48279: F, t17748: F, t10235: F, t13851: F, t4531: F, t48021: F, t48024: F, t48030: F, t48044: F, t48048: F, t48052: F, t48357: F) -> F {
    let t61279 = t10236 * t17635;
    let t61288 = t2986 * t13847 * t13835;
    let t61291 = t2986 * t48279 * t13839;
    let t61294 = t2986 * t13847 * t17748;
    let t61301 = -F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4531 * t48357 - F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t10235 * t61279 + F::cast_from(0.24691358024691358024e-3_f64) * t48021 + F::cast_from(0.37037037037037037036e-3_f64) * t48024 - F::cast_from(0.37037037037037037036e-3_f64) * t48030 + F::cast_from(0.11111111111111111111e-2_f64) * t48044 + F::cast_from(0.74074074074074074073e-3_f64) * t61288 - F::cast_from(0.49382716049382716048e-3_f64) * t61291 - F::cast_from(0.37037037037037037036e-3_f64) * t61294 + F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t13851 * t13835 - F::cast_from(0.37037037037037037036e-3_f64) * t48048 - F::cast_from(0.18518518518518518518e-3_f64) * t48052;
    t61301
}
