//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 533/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk533<F: Float>(t2222: F, t730: F, t200: F, t202: F, t692: F, t725: F, t650: F, t698: F, t169: F, t697: F, t164: F, t704: F, t705: F, t2187: F, t2190: F, t2193: F, t2197: F, t2199: F, t2202: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2224 = 0.24415263074675393405e-3 * t730 * t2222;
    let t2225 = 1.0 / t200;
    let t2232 = 1.0 / t202;
    let t2245 = t692 * t725;
    let t2250 = t650 * t698;
    let t2254 = t697 * t169;
    let t2255 = 1.0 / t2254;
    let t2256 = t164 * t2255;
    let t2257 = t704 * t704;
    let t2258 = t2257 * t705;
    let t2267 = -0.78438333333333333333e0 * t2187 + 0.15687666666666666667e1 * t2190 + 0.68863333333333333333e0 * t2193 + 0.14025833333333333333e0 * t2197 + 0.28051666666666666667e0 * t2199 + 0.17365833333333333333e0 * t2202;
    (t2224, t2225, t2232, t2245, t2250, t2255, t2256, t2257, t2258, t2267)
}
