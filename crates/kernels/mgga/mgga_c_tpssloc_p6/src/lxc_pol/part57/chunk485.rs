//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 485/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk485<F: Float>(t1118: F, t6020: F, t1099: F, t3315: F, t5988: F, t3313: F, t3319: F, t4721: F, t5973: F, t5977: F, t5981: F, t1682: F) -> (F, F, F, F) {
    let t6021 = t6020 * t1118;
    let t6023 = F::new(1.0) * t1099 * t6021;
    let t6024 = t5988 * t3315;
    let t6026 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t6024;
    let t6031 = t3319 - F::cast_from(0.11415555555555555555e-1_f64) * t4721 - F::cast_from(0.11415555555555555555e-1_f64) * t5973 + F::cast_from(0.34246666666666666666e-1_f64) * t5977 + F::cast_from(0.17123333333333333333e-1_f64) * t5981;
    let t6036 = t1682 * t1682;
    (t6023, t6026, t6031, t6036)
}
