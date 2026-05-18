//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 806/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk806<F: Float>(t12200: F, t1614: F, t262: F, t3068: F, t41015: F, t739: F, t7577: F, t14125: F, t68440: F, t9205: F, t14224: F, t8576: F) -> (F, F, F, F) {
    let t74487 = t12200 * t3068 * t262 * t1614;
    let t74491 = F::new(0.5987120850931904282e-1) * t739 * t7577 * t41015;
    let t74495 = t68440 * t14125 * t9205;
    let t74497 = t8576 * t14224;
    (t74487, t74491, t74495, t74497)
}
