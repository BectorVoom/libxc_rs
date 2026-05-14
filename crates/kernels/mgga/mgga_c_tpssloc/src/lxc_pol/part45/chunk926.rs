//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 926/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk926<F: Float>(t66940: F, t8657: F, t111: F, t8646: F, t12524: F, t31814: F, t2039: F, t22479: F, t3941: F, t112: F, t31781: F, t7230: F, t1873: F, t84078: F, t94165: F, t24462: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115983 = 54.0 * t66940 * t8657;
    let t115984 = t8646 * t111;
    let t115990 = 54.0 * t12524 * t31814;
    let t115995 = 27.0 * t3941 * t2039 * t22479;
    let t115996 = t31781 * t112;
    let t116000 = 0.135e2 * t7230 * t22479;
    let t116004 = 0.135e2 * t84078 * t1873;
    let t116006 = 27.0 * t94165 * t1873;
    let t116008 = 27.0 * t24462 * t6534;
    (t115983, t115984, t115990, t115995, t115996, t116000, t116004, t116006, t116008)
}
