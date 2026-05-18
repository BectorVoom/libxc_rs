//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 926/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk926<F: Float>(t275: F, t277: F, t8662: F, t267: F, t270: F, t279: F, t8660: F, t2529: F, t844: F, t269: F, t2532: F, t284: F) -> (F, F, F, F, F, F, F) {
    let t8664 = t275 * t8662 * t277;
    let t8665 = F::new(0.36514074074074074075e0) * t8664;
    let t8678 = F::new(1.0)/pow_3_2::<f64>(t267);
    let t8684 = F::new(1.0) / t270 / t279 / F::new(4.0);
    let t8687 = F::new(28.0) / F::new(27.0) * t8660;
    let t8709 = F::new(1.0) / t2529 / t844;
    let t8710 = t269 * t8709;
    let t8712 = F::new(1.0) / t2532 / t284;
    (t8664, t8665, t8678, t8684, t8687, t8710, t8712)
}
