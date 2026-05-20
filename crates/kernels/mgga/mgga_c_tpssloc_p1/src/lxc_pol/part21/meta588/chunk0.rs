//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2327/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2327<F: Float>(t12283: F, t6396: F, t19871: F, t3805: F, t3807: F, t16306: F, t6394: F, t16305: F, t16225: F, t16311: F, t1825: F, t5308: F) -> (F, F, F, F, F) {
    let t19879 = t12283 * t6396;
    let t19882 = t3805 * t19871 * t3807;
    let t19885 = t16306 * t6394;
    let t19886 = t16305 * t19885;
    let t19889 = t16311 * t16225;
    let t19890 = t16305 * t19889;
    let t19893 = t1825 * t5308;
    (t19879, t19882, t19886, t19890, t19893)
}
