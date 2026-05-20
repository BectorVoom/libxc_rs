//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2415/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2415<F: Float>(t14363: F, t942: F, t10760: F, t10806: F, t10814: F, t14329: F, t14332: F, t1569: F, t2856: F, t2925: F, t42117: F, t4411: F, t4434: F, t49268: F, t49271: F, t49273: F, t49276: F, t49278: F, t49280: F, t49282: F, t49285: F, t49305: F, t49318: F, t49332: F, t49345: F, t49359: F, t49372: F, t49386: F, t49397: F, t924: F, t932: F, t952: F) -> F {
    let t49404 = t14363 * t942;
    let t49409 = -t49268 - t49271 - t49273 - t49276 - t49278 - t49280 - t49282 + F::new(1.0) * t4411 * t10806 + F::cast_from(0.2069040516770936012e4_f64) * t49285 * t10814 + F::new(1.0) * t42117 * t1569 + F::new(3.0) * t10760 * t4434 + F::new(3.0) * t2856 * t14329 + F::new(1.0) * t924 * (t49305 + t49318 + t49332 + t49345 + t49359 + t49372 + t49386 + t49397) * t932 + F::cast_from(0.17544670867903938621e1_f64) * t49404 * t952 + F::cast_from(0.17544670867903938621e1_f64) * t14332 * t2925;
    t49409
}
