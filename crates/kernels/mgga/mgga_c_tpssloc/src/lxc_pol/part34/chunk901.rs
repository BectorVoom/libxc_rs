//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 901/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk901<F: Float>(t1988: F, t22716: F, t22724: F, t6898: F, t225: F, t3886: F, t25: F, t2752: F, t1887: F, t6581: F) -> (F, F, F, F, F) {
    let t22923 = t22716 * t1988;
    let t22925 = t22724 * t6898;
    let t22933 = t225 * t3886;
    let t22960 = t2752 * t25;
    let t22986 = t6581 * t1887;
    (t22923, t22925, t22933, t22960, t22986)
}
