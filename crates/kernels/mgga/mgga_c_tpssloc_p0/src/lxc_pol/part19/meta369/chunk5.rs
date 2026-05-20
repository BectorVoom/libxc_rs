//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1365/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1365<F: Float>(t10250: F, t2970: F, t973: F, t10195: F, t10231: F, t1005: F, t10375: F, t10475: F, t42342: F, t42345: F, t2770: F, t283: F) -> (F, F, F, F, F) {
    let t43374 = t973 * t2970 * t10250;
    let t43377 = t973 * t10231 * t10195;
    let t43382 = t1005 * t10375;
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = F::new(1.0) / t283 / t2770;
    (t43374, t43377, t43382, t43385, t43398)
}
