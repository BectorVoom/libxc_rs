//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1047/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1047<F: Float>(t649: F, t8319: F, t510: F, t1266: F, t8320: F, t8301: F, t9231: F, t645: F, t8307: F, t8513: F, t31: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t30991 = t649 * t8319;
    let t30993 = F::cast_from(2.0_f64) * t30991 * t510;
    let t30995 = F::cast_from(2.0_f64) * t8320 * t1266;
    let t31000 = t9231 * t8301;
    let t31005 = t8307 * t645;
    let t31006 = t8513 * t31005;
    let t31011 = t8307 * t31;
    let t31013 = t8513 * t31011 * t607;
    (t30991, t30993, t30995, t31000, t31006, t31011, t31013)
}
