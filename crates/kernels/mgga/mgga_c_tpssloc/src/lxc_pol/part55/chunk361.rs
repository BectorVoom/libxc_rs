//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 361/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk361<F: Float>(t1716: F, t974: F, t1173: F, t1174: F, t1706: F, t1710: F, t463: F) -> (F,) {
    let t1717 = t974 * t1716;
    let t1720 = -0.22222222222222222222e-2 * t1706 * t463 + t1173 - 0.27777777777777777777e-3 * t1174 * t1710 - 0.83333333333333333332e-3 * t1174 * t1717;
    (t1720,)
}
