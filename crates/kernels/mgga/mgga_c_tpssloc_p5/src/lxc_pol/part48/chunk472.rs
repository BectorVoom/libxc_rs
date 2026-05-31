//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 472/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk472<F: Float>(t3493: F, t475: F, t1214: F, t248: F, t3030: F, t466: F, t3032: F, t1208: F, t476: F) -> (F, F, F, F) {
    let t3494 = t3493 * t475;
    let t3496 = t248 * t1214 * t3494;
    let t3499 = t466 * t3030;
    let t3500 = t3499 * t3032;
    let t3502 = F::cast_from(1.0_f64) / t1208 / t476;
    (t3496, t3499, t3500, t3502)
}
