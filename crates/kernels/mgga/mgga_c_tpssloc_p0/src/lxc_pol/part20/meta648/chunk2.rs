//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2381/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2381<F: Float>(t13716: F, t2932: F, t10632: F, t4471: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47705: F, t48085: F, t48087: F, t48090: F, t48092: F) -> (F, F, F) {
    let t48883 = t13716 * t2932;
    let t48890 = t4471 * t10632;
    let t48907 = -F::cast_from(0.98587999999999999998e0_f64) * t48085 + F::cast_from(0.98587999999999999998e0_f64) * t48087 + F::cast_from(0.49293999999999999999e0_f64) * t48090 - F::cast_from(0.82156666666666666668e-1_f64) * t48092 - F::cast_from(0.88582716049382716048e0_f64) * t47681 + F::cast_from(0.35876000000000000001e1_f64) * t47686 - F::cast_from(0.59793333333333333333e0_f64) * t47691 - F::cast_from(0.59793333333333333333e0_f64) * t47695 - F::cast_from(0.19931111111111111111e0_f64) * t47699 - F::cast_from(0.53814000000000000001e1_f64) * t47703 + F::cast_from(0.79724444444444444445e0_f64) * t47705;
    (t48883, t48890, t48907)
}
