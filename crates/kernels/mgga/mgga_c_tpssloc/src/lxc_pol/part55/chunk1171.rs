//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1171/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1171<F: Float>(t606: F, t7540: F, t2752: F, t32885: F, t1877: F, t2219: F, t8370: F, t25365: F, t25373: F, t1408: F, t6665: F, t1530: F) -> (F, F, F, F, F, F) {
    let t118393 = t606 * t7540;
    let t118399 = t32885 * t2752;
    let t118406 = t1877 * t8370 * t2219;
    let t118407 = t25373 * t25365;
    let t118410 = t1408 * t6665;
    let t118413 = t1530 * t6665;
    (t118393, t118399, t118406, t118407, t118410, t118413)
}
