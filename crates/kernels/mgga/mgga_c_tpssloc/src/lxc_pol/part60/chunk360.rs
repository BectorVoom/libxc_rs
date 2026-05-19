//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 360/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk360<F: Float>(t265: F, t504: F, t2148: F, t462: F, t2144: F, t493: F, t2121: F, t470: F, t1241: F, t1238: F, t2124: F, t2145: F, t498: F, t1256: F, t193: F, t1964: F, t336: F) -> (F, F, F, F, F, F) {
    let t505 = t265 < t504;
    let t2149 = t462 * t2148;
    let t2152 = t493 * t2144;
    let t2154 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t2149 + t470 * t2152;
    let t2155 = t1241 * t2154;
    let t2157 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t2124 + t2145 * t498 - t1238 * t2155;
    let t2161 = piecewise3::<F>(t505, t193 * t336 * t2157 * t1256, t1964);
    (t2149, t2152, t2154, t2155, t2157, t2161)
}
