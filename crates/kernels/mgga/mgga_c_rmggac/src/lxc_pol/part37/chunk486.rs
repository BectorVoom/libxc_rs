//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 486/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk486<F: Float>(t3046: F, t3810: F, t2566: F, t2048: F, t637: F, t854: F, t1322: F) -> (F, F, F, F) {
    let t13931 = t3810 * t3046;
    let t13932 = t13931 * t2566;
    let t13935 = t854 * t2048 * t637;
    let t13937 = t3810 * t1322;
    (t13931, t13932, t13935, t13937)
}
