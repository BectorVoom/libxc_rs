//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 968/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk968<F: Float>(t24574: F, t7288: F, t225: F, t7306: F, t3640: F, t7394: F, t11947: F, t2157: F, t111: F, t7263: F) -> (F, F, F, F, F) {
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24905 = t7394 * t3640;
    let t24909 = t2157 * t11947;
    let t24932 = t7263 * t111;
    (t24891, t24893, t24905, t24909, t24932)
}
