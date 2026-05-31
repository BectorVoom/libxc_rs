//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1013/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1013<F: Float>(t33234: F, t7461: F, t33617: F, t4028: F, t7458: F, t652: F, t7467: F, t7890: F, t33214: F, t7802: F, t29211: F, t8526: F) -> (F, F, F, F, F, F) {
    let t128406 = F::cast_from(4.0_f64) * t33234 * t7461;
    let t128413 = F::cast_from(4.0_f64) * t4028 * t33617;
    let t128415 = F::cast_from(4.0_f64) * t7458 * t33617;
    let t128418 = F::cast_from(4.0_f64) * t652 * t7890 * t7467;
    let t128420 = F::cast_from(4.0_f64) * t33214 * t7802;
    let t128422 = F::cast_from(2.0_f64) * t8526 * t29211;
    (t128406, t128413, t128415, t128418, t128420, t128422)
}
