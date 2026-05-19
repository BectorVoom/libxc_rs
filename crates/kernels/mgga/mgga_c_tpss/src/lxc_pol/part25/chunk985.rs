//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 985/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk985<F: Float>(t12677: F, t1170: F, t5393: F, t1173: F, t12689: F, t12692: F, t9841: F, t3184: F, t5371: F, t5328: F, t9856: F, t3282: F, t4578: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13568 = F::cast_from(0.48830526149350786811e-3_f64) * t12677;
    let t13569 = t1170 * t5393;
    let t13570 = F::new(4.0) * t13569;
    let t13571 = t1173 * t5393;
    let t13572 = F::new(4.0) * t13571;
    let t13573 = F::new(16.0) * t12689;
    let t13574 = F::new(2.0) * t12692;
    let t13575 = F::cast_from(0.10843581300301739842e-1_f64) * t9841;
    let t13576 = t3184 * t5371;
    let t13583 = t9856 * t5328;
    let t13588 = t3282 * t4578;
    (t13568, t13570, t13572, t13573, t13574, t13575, t13576, t13583, t13588)
}
