//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1052/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1052<F: Float>(t66940: F, t8657: F, t111: F, t8646: F, t12524: F, t31814: F, t2039: F, t22479: F, t3941: F, t112: F, t31781: F, t7230: F) -> (F, F, F, F, F, F) {
    let t115983 = F::new(54.0) * t66940 * t8657;
    let t115984 = t8646 * t111;
    let t115990 = F::new(54.0) * t12524 * t31814;
    let t115995 = F::new(27.0) * t3941 * t2039 * t22479;
    let t115996 = t31781 * t112;
    let t116000 = F::new(0.135e2) * t7230 * t22479;
    (t115983, t115984, t115990, t115995, t115996, t116000)
}
