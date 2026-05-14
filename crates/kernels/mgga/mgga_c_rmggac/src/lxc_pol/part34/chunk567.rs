//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 567/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk567<F: Float>(t27: F, t615: F, t271: F, t71: F, t198: F, t202: F, t3127: F, t14113: F, t14123: F) -> (F, F, F, F, F) {
    let t17881 = t27 * t615;
    let t20925 = t271 * t71;
    let t21052 = t198 * t202;
    let t21060 = t198 * t3127;
    let t21708 = t14113 * t14123;
    (t17881, t20925, t21052, t21060, t21708)
}
