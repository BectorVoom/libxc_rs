//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 734/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk734<F: Float>(t14551: F, t7508: F, t68735: F, t235: F, t29837: F, t698: F, t2046: F, t2050: F, t2232: F, t31: F, t68757: F, t68791: F) -> (F, F, F, F, F, F) {
    let t70948 = t7508 * t14551;
    let t71005 = F::new(0.54934029498967360725e-3) * t68735;
    let t71007 = t235 * t29837 * t698;
    let t71021 = t2046 * t2050 * t2232 * t31;
    let t71033 = F::new(0.34547904762044099522e0) * t68757;
    let t71042 = F::new(0.86737941314158990616e-4) * t68791;
    (t70948, t71005, t71007, t71021, t71033, t71042)
}
