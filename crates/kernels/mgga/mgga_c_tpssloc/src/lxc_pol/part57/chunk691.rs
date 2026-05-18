//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 691/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk691<F: Float>(t3787: F, t6604: F, t1988: F, t22716: F, t22724: F, t6898: F, t225: F, t3886: F, t1914: F, t193: F, t201: F, t25: F, t2752: F) -> (F, F, F, F, F, F, F, F) {
    let t22897 = t6604 * t3787;
    let t22923 = t22716 * t1988;
    let t22924 = F::new(0.63969658155208805863e-1) * t22923;
    let t22925 = t22724 * t6898;
    let t22926 = F::new(0.26044789391763585244e-1) * t22925;
    let t22933 = t225 * t3886;
    let t22959 = t193 * t201 * t1914;
    let t22960 = t2752 * t25;
    (t22897, t22923, t22924, t22925, t22926, t22933, t22959, t22960)
}
