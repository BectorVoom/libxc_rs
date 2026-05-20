//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1270/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270<F: Float>(t18375: F, t5002: F, t1730: F, t19032: F, t1017: F, t1207: F, t1210: F, t22173: F, t372: F, t471: F, t479: F, t15507: F, t19095: F) -> (F, F, F, F, F) {
    let t72366 = t5002 * t18375;
    let t72384 = t1730 * t19032;
    let t72389 = t1207 * t1210 * t22173 * t1017;
    let t72398 = t471 * t479 * t22173 * t372;
    let t72403 = t15507 * t19095;
    (t72366, t72384, t72389, t72398, t72403)
}
