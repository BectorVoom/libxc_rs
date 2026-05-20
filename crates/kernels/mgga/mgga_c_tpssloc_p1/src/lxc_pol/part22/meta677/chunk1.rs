//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2238/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238<F: Float>(t1041: F, t13969: F, t17975: F, t17687: F, t14085: F, t4571: F, t13765: F, t13995: F, t18086: F, t3069: F, t10952: F, t17655: F) -> (F, F, F, F, F, F) {
    let t61919 = t1041 * t13969 * t17975;
    let t61923 = t1041 * t13969 * t17687;
    let t61929 = t14085 * t4571;
    let t61940 = t13995 * t13765;
    let t61950 = t18086 * t3069;
    let t61975 = t10952 * t17655;
    (t61919, t61923, t61929, t61940, t61950, t61975)
}
