//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1184/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1184<F: Float>(t2031: F, t83718: F, t2240: F, t240: F, t33: F, t6492: F, t2244: F, t63: F, t23993: F, t6495: F, t2032: F, t22493: F, t22537: F, t23963: F, t24001: F, t6486: F, t7035: F, t83717: F, t83734: F, t83748: F, t83822: F) -> F {
    let t84237 = t2031 * t83718;
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84245 = t2240 * t2244 * t63;
    let t84248 = t6495 * t23993;
    let t84258 = t22493 * t7035 + t6486 * t24001 + F::cast_from(30.0_f64) * t23963 * t83734 - F::cast_from(60.0_f64) * t83717 * t84237 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t84242 + F::cast_from(10.0_f64) * t84245 * t6492 - F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t84248 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t83822 * t2032 - F::cast_from(2.0_f64) * t22537 * t7035 - F::cast_from(2.0_f64) * t6495 * t24001 - F::cast_from(2.0_f64) * t83748 * t2032;
    t84258
}
