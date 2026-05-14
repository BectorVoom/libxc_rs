//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1085/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1085<F: Float>(t12300: F, t3853: F, t12305: F, t3866: F, t12238: F, t68: F, t1340: F, t10021: F, t1336: F, t1339: F, t1354: F, t12365: F, t3858: F, t12379: F, t3799: F, t12384: F, t3777: F) -> (F, F, F, F, F, F, F, F) {
    let t40114 = t12300 * t3853;
    let t40116 = t3866 * t12305;
    let t40118 = t12238 * t68;
    let t40119 = t40118 * t1340;
    let t40123 = t1336 * t1339 * t10021;
    let t40124 = t40123 * t1354;
    let t40126 = t12365 * t3858;
    let t40128 = t3799 * t12379;
    let t40130 = t3777 * t12384;
    (t40114, t40116, t40118, t40119, t40124, t40126, t40128, t40130)
}
