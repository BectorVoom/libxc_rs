//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 872/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk872<F: Float>(t14368: F, t15223: F, t15227: F, t70279: F, t1550: F, t2060: F, t40983: F, t69894: F, t27: F, t9151: F, t16064: F, t69609: F) -> (F, F, F, F, F) {
    let t75596 = t14368 * t15223;
    let t75598 = t70279 * t15227;
    let t75602 = F::new(0.5987120850931904282e-1) * t1550 * t2060 * t40983;
    let t75607 = F::new(0.79828278012425390427e-1) * t69894;
    let t75609 = t27 * t9151;
    let t75611 = t69609 * t16064 * t75609;
    (t75596, t75598, t75602, t75607, t75611)
}
