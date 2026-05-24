//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 786/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk786<F: Float>(t36541: F, t674: F, t7269: F, t7508: F, t2084: F, t2145: F, t27: F, t866: F, t1347: F, t2153: F, t1987: F, t7939: F) -> (F, F, F, F, F) {
    let t36542 = t36541 * t674;
    let t36590 = t7508 * t7269;
    let t36594 = t2145 * t27 * t2084 * t866;
    let t36601 = t1347 * t2153;
    let t36610 = t7939 * t1987;
    (t36542, t36590, t36594, t36601, t36610)
}
