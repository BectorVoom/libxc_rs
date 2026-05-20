//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1431/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431<F: Float>(t423: F, t78266: F, t78278: F, t21961: F, t51249: F, t11275: F, t3315: F, t78129: F, t6068: F) -> (F, F, F, F) {
    let t78281 = F::new(0.621814e-1) * (t78266 + t78278) * t423;
    let t78283 = F::cast_from(0.3859675079686208416e3_f64) * t51249 * t21961;
    let t78286 = F::cast_from(0.57895126195293126241e3_f64) * t11275 * t78129 * t3315;
    let t78287 = t6068 * t6068;
    (t78281, t78283, t78286, t78287)
}
