//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1176/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1176<F: Float>(t215: F, t39933: F, t535: F, t12227: F, t9577: F, t116: F, t557: F, t212: F, t2586: F, t3734: F, t12225: F, t3719: F) -> (F, F, F, F) {
    let t40350 = F::cast_from(0.14979423868312757201e0_f64) * t39933 * t535 * t215;
    let t40351 = t9577 * t12227;
    let t40353 = t557 * t116;
    let t40356 = t2586 * t40353 * t212 * t3734;
    let t40360 = t2586 * t12225 * t212 * t3719;
    (t40350, t40351, t40356, t40360)
}
