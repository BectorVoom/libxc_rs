//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1421/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421<F: Float>(t3266: F, t3307: F, t3313: F, t1119: F, t11269: F, t3264: F, t11190: F, t3316: F, t11185: F, t11407: F, t1117: F, t3315: F) -> (F, F, F, F, F) {
    let t43994 = F::new(36.0) * t3313 * t3266 * t3307;
    let t43997 = F::new(8.0) * t3264 * t1119 * t11269;
    let t44000 = F::cast_from(0.57895126195293126241e3_f64) * t11190 * t3316 * t3307;
    let t44002 = F::cast_from(0.1929837539843104208e3_f64) * t11185 * t11407;
    let t44006 = F::cast_from(0.64327917994770140268e2_f64) * t3313 * t11269 * t3315 * t1117;
    (t43994, t43997, t44000, t44002, t44006)
}
