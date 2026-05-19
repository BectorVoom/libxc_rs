//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 942/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk942<F: Float>(t12132: F, t17: F, t3826: F, t592: F, t1285: F, t2225: F, t2371: F, t3691: F, t1294: F, t9494: F, t2535: F, t12121: F, t12123: F, t12125: F, t12128: F, t12131: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F) {
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12135 = F::new(24.0) * t12134;
    let t12136 = t2225 * t1285;
    let t12137 = F::new(60.0) * t12136;
    let t12138 = t3691 * t2371;
    let t12139 = F::cast_from(0.35089341735807877242e1_f64) * t12138;
    let t12141 = F::cast_from(0.10254018858216406658e4_f64) * t1294 * t9494;
    let t12142 = t3691 * t2535;
    let t12143 = F::cast_from(0.17544670867903938621e1_f64) * t12142;
    let t12144 = t12121 + t12123 + t12125 + t12128 + t12131 + t12133 - t12135 + t12137 + t9853 + t12139 + t9859 - t12141 - t12143;
    (t12133, t12135, t12137, t12139, t12141, t12143, t12144)
}
