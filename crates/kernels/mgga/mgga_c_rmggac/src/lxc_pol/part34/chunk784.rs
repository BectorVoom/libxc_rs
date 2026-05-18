//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 784/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk784<F: Float>(t21713: F, t74120: F, t9189: F, t21714: F, t9193: F, t9197: F, t14125: F, t68421: F, t73699: F, t14124: F, t236: F, t498: F, t598: F, t68422: F) -> (F, F, F, F, F) {
    let t74122 = t21713 * t74120 * t9189;
    let t74125 = t21713 * t21714 * t9193;
    let t74128 = t21713 * t21714 * t9197;
    let t74131 = t68421 * t14125 * t73699;
    let t74137 = t14124 * t68422 * t236 * t598 * t498;
    (t74122, t74125, t74128, t74131, t74137)
}
