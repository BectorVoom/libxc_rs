//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 691/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk691<F: Float>(t13962: F, t3056: F, t7385: F, t7301: F, t7305: F, t34: F, t79: F, t34750: F, t637: F, t26007: F, t271: F, t71: F) -> (F, F, F, F, F, F) {
    let t69085 = t3056 * t13962 * t7385;
    let t69091 = t3056 * t13962 * t7301;
    let t69094 = t3056 * t13962 * t7305;
    let t69097 = F::new(1.0) / t34 / t79;
    let t69101 = t34750 * t637;
    let t69102 = t26007 * t69097 * t271 * t71 * t69101;
    (t69085, t69091, t69094, t69097, t69101, t69102)
}
