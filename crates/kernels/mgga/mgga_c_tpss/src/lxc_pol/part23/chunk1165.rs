//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1165/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1165<F: Float>(t30: F, t259: F, t379: F, t18592: F, t645: F, t547: F, t2105: F, t5772: F, t117: F, t18403: F, t18230: F, t18066: F, t1867: F, t1992: F, t45: F, t581: F, t5994: F, t1872: F, t3025: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t18593 = t18592 * t645;
    let t18595 = 12.0 * t547 * t18593;
    let t18596 = t5772 * t2105;
    let t18598 = 6.0 * t547 * t18596;
    let t18599 = t117 * t18403;
    let t18601 = 3.0 * t547 * t18599;
    let t19057 = piecewise3(t380, 0.0, t18230);
    let t19064 = piecewise3(t120, t18066, t19057 * t45 / 2.0 + t5994 * t581 + t1867 * t1992 / 2.0);
    let t19066 = t1872 * t3025 / 432.0;
    (t18593, t18595, t18596, t18598, t18599, t18601, t19057, t19064, t19066)
}
