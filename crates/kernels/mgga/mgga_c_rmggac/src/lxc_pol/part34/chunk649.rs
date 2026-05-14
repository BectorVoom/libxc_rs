//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 649/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk649<F: Float>(t2039: F, t2232: F, t270: F, t638: F, t1338: F, t703: F, t2046: F, t2050: F, t2244: F, t31: F, t2144: F, t2227: F, t507: F, t69265: F, t69267: F, t69270: F) -> (F, F, F, F, F, F, F) {
    let t71214 = t638 * t2039 * t2232 * t270;
    let t71218 = t638 * t2039 * t703 * t1338;
    let t71219 = 0.15243824895787514157e-3 * t71218;
    let t71222 = t2046 * t2050 * t2244 * t31;
    let t71229 = t507 * t2144 * t2227;
    let t71269 = 0.16852636469289804646e0 * t69265;
    let t71270 = 0.56054872888033565654e-2 * t69267;
    let t71271 = 0.11515968254014699841e0 * t69270;
    (t71214, t71219, t71222, t71229, t71269, t71270, t71271)
}
