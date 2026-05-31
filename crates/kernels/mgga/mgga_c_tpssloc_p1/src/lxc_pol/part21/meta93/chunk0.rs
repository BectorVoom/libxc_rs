//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 659/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk659<F: Float>(t659: F, t2341: F, t2248: F, t95: F, t102: F) -> (F, F, F, F) {
    let t2342 = t659 * t659;
    let t2343 = t2341 * t2342;
    let t2346 = t95 * t2248;
    let t2349 = F::cast_from(1.0_f64) / t102;
    (t2342, t2343, t2346, t2349)
}
