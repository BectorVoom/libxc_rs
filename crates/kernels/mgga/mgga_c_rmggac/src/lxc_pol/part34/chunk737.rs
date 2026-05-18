//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 737/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk737<F: Float>(t14580: F, t892: F, t69027: F, t2145: F, t3224: F, t7581: F, t388: F, t703: F, t7933: F, t7934: F, t2039: F, t2232: F, t270: F, t638: F) -> (F, F, F, F, F) {
    let t71198 = t892 * t14580;
    let t71204 = F::new(0.22800128353348964998e-6) * t69027;
    let t71206 = t2145 * t3224 * t7581;
    let t71207 = F::new(0.33335697577410973224e-1) * t71206;
    let t71210 = t7933 * t7934 * t388 * t703;
    let t71214 = t638 * t2039 * t2232 * t270;
    (t71198, t71204, t71207, t71210, t71214)
}
