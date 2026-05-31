//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1013/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1013<F: Float>(t12524: F, t31817: F, t66940: F, t8657: F, t31814: F, t2039: F, t22479: F, t3941: F, t7230: F, t1873: F, t84078: F, t94165: F) -> (F, F, F, F, F, F, F) {
    let t115980 = F::cast_from(54.0_f64) * t12524 * t31817;
    let t115983 = F::cast_from(54.0_f64) * t66940 * t8657;
    let t115990 = F::cast_from(54.0_f64) * t12524 * t31814;
    let t115995 = F::cast_from(27.0_f64) * t3941 * t2039 * t22479;
    let t116000 = F::cast_from(0.135e2_f64) * t7230 * t22479;
    let t116004 = F::cast_from(0.135e2_f64) * t84078 * t1873;
    let t116006 = F::cast_from(27.0_f64) * t94165 * t1873;
    (t115980, t115983, t115990, t115995, t116000, t116004, t116006)
}
