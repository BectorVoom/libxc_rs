//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2434/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434<F: Float>(t17513: F, t49489: F, t10661: F, t21253: F, t912: F, t2842: F, t4395: F, t5695: F, t10702: F, t21268: F, t10817: F, t21315: F) -> (F, F, F, F, F) {
    let t69288 = F::cast_from(0.2894756309764656312e3_f64) * t49489 * t17513;
    let t69291 = F::new(24.0) * t10661 * t21253 * t912;
    let t69294 = F::new(18.0) * t2842 * t5695 * t4395;
    let t69297 = F::cast_from(0.57895126195293126241e3_f64) * t10702 * t21268 * t912;
    let t69299 = F::new(6.0) * t10817 * t21315;
    (t69288, t69291, t69294, t69297, t69299)
}
