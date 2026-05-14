//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 861/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk861<F: Float>(t2174: F, t7945: F, t34175: F, t580: F, t2169: F, t7961: F, t34194: F, t576: F, t33153: F, t33151: F, t119878: F, t1409: F, t1410: F, t1433: F, t2240: F, t32: F, t5392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t125067 = t7945 * t2174;
    let t125068 = t34175 * t580;
    let t125069 = t2169 * t7961;
    let t125071 = t576 * t34194;
    let t126035 = 4.0 * t33153;
    let t126036 = 4.0 * t33151;
    let t126065 = t119878 * t1409;
    let t126073 = t1410 * t1433;
    let t126091 = t2240 * t32 * t5392;
    (t125067, t125068, t125069, t125071, t126035, t126036, t126065, t126073, t126091)
}
