//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 674/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk674<F: Float>(t28: F, t518: F, t1302: F, t3231: F, t3673: F, t3710: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t3711 = 1.0 / t518;
    let t3717 = piecewise3(t29, 0.0, -2.0 / 9.0 * t3711 * t3673 + 2.0 / 3.0 * t1302 * t3231);
    let t3719 = t3710 / 2.0 + t3717 / 2.0;
    (t3711, t3719)
}
