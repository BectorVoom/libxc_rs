//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 891/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk891<F: Float>(t22607: F, t8641: F, t1983: F, t31669: F, t6999: F, t7015: F, t84033: F, t12524: F, t31817: F, t66940: F, t8657: F, t31814: F, t2039: F, t22479: F, t3941: F, t7230: F) -> (F, F, F, F, F, F, F, F) {
    let t115965 = t22607 * t8641;
    let t115968 = 2.0 * t1983 * t31669 * t6999;
    let t115978 = 54.0 * t84033 * t7015;
    let t115980 = 54.0 * t12524 * t31817;
    let t115983 = 54.0 * t66940 * t8657;
    let t115990 = 54.0 * t12524 * t31814;
    let t115995 = 27.0 * t3941 * t2039 * t22479;
    let t116000 = 0.135e2 * t7230 * t22479;
    (t115965, t115968, t115978, t115980, t115983, t115990, t115995, t116000)
}
