//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2093/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093<F: Float>(t374: F, t485: F, t486: F, t9697: F, t11778: F, t121: F, t1229: F, t204: F, t1090: F, t1227: F, t248: F, t11880: F, t44690: F) -> (F, F, F, F, F) {
    let t45250 = F::new(7.0) / F::new(31104.0) * t485 * t374 * t9697 * t486;
    let t45268 = t121 * t11778;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45326 = t44690 * t11880;
    (t45250, t45268, t45293, t45296, t45326)
}
