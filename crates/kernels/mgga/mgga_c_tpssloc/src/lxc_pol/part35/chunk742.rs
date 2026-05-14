//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 742/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk742<F: Float>(t1530: F, t25: F, t1408: F, t1877: F, t1915: F, t2522: F, t6670: F, t7476: F, t7541: F, t1409: F, t3: F, t1484: F, t202: F, t7540: F, t193: F, t870: F) -> (F, F, F, F) {
    let t7545 = t25 * t1530;
    let t7552 = 3.0 / 2.0 * t2522 * t7476 + t1877 * t7541 * t25 / 2.0 - t1877 * t6670 * t7545 / 2.0 + t1877 * t1915 * t1408 / 2.0;
    let t7573 = t3 * t1409;
    let t7634 = t1915 * t1484;
    let t7637 = t202 * t7540;
    let t7642 = -t1530 * t1877 * t6670 + t193 * t7637 * t870 + 3.0 * t2522 * t7634;
    (t7545, t7552, t7573, t7642)
}
