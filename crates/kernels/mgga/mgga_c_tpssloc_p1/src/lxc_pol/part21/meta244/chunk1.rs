//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1434/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1434<F: Float>(t3313: F, t6024: F, t3319: F, t4721: F, t5973: F, t5977: F, t5981: F, t1682: F, t1137: F) -> (F, F, F, F) {
    let t6026 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t6024;
    let t6031 = t3319 - F::cast_from(0.11415555555555555555e-1_f64) * t4721 - F::cast_from(0.11415555555555555555e-1_f64) * t5973 + F::cast_from(0.34246666666666666666e-1_f64) * t5977 + F::cast_from(0.17123333333333333333e-1_f64) * t5981;
    let t6036 = t1682 * t1682;
    let t6037 = t6036 * t1137;
    (t6026, t6031, t6036, t6037)
}
