//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1046/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046<F: Float>(t1113: F, t21749: F, t136: F, t11195: F, t11204: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F) -> (F, F, F) {
    let t21750 = t1113 * t21749;
    let t21751 = t136 * t21750;
    let t21753 = -t11195 - F::cast_from(0.16431333333333333333e0_f64) * t18512 + F::cast_from(0.19931111111111111111e0_f64) * t18203 - F::cast_from(0.59793333333333333333e0_f64) * t18219 - F::cast_from(0.29896666666666666667e0_f64) * t18229 + F::cast_from(0.5477111111111111111e-1_f64) * t18494 - F::cast_from(0.32862666666666666666e0_f64) * t18505 - F::cast_from(0.28483875e1_f64) * t21739 + F::cast_from(0.46074375e0_f64) * t21741 - t11204 + F::cast_from(0.39862222222222222223e0_f64) * t14702 + F::cast_from(0.27385555555555555556e0_f64) * t14766 - F::cast_from(0.82156666666666666668e-1_f64) * t21747 + F::cast_from(0.49293999999999999999e0_f64) * t21751;
    (t21750, t21751, t21753)
}
