//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 908/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk908<F: Float>(t2532: F, t284: F, t8660: F, t2480: F, t841: F, t2617: F, t894: F, t2620: F, t317: F, t314: F, t8664: F, t2574: F, t872: F, t2573: F, t301: F, t296: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8712 = 1.0 / t2532 / t284;
    let t8723 = 0.55403703703703703703e-1 * t8660;
    let t8737 = t841 * t2480;
    let t8749 = 1.0 / t2617 / t894;
    let t8752 = 1.0 / t2620 / t317;
    let t8756 = 0.28842592592592592592e-1 * t8660;
    let t8772 = 1.0 / t2617 / t314;
    let t8796 = 0.93932222222222222223e0 * t8660;
    let t8797 = 0.36793333333333333333e0 * t8664;
    let t8842 = t872 * t2574;
    let t8846 = 1.0 / t2573 / t301;
    let t8847 = t296 * t8846;
    (t8712, t8723, t8737, t8749, t8752, t8756, t8772, t8796, t8797, t8842, t8847)
}
