//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 736/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk736<F: Float>(t14584: F, t507: F, t14588: F, t69016: F, t14580: F, t892: F, t2145: F, t3224: F, t7581: F, t388: F, t703: F, t7933: F, t7934: F) -> (F, F, F, F, F, F) {
    let t71163 = t507 * t14584;
    let t71167 = t507 * t14588;
    let t71196 = F::cast_from(0.16263363996404810741e-4_f64) * t69016;
    let t71198 = t892 * t14580;
    let t71206 = t2145 * t3224 * t7581;
    let t71207 = F::cast_from(0.33335697577410973224e-1_f64) * t71206;
    let t71210 = t7933 * t7934 * t388 * t703;
    (t71163, t71167, t71196, t71198, t71207, t71210)
}
