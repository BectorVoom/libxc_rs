//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 847/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk847<F: Float>(t23096: F, t23106: F, t23108: F, t23114: F, t23119: F, t25085: F, t25087: F, t25089: F, t25091: F, t25095: F, t25099: F, t1894: F, t236: F, t4119: F, t6591: F, t23062: F, t7497: F) -> (F, F, F) {
    let t25103 = t23096 - t23106 + t25085 / 768.0 + t25087 / 384.0 - t25089 / 1536.0 + t25091 / 384.0 + 0.40372756094140390854e-3 * t25095 + t23108 + 0.12111826828242117256e-2 * t25099 + 0.33643963411783659045e-4 * t23114 - 7.0 / 2304.0 * t23119;
    let t25106 = t1894 * t236 * t4119;
    let t25107 = t6591 * t25106;
    let t25109 = t23062 * t7497;
    (t25103, t25107, t25109)
}
