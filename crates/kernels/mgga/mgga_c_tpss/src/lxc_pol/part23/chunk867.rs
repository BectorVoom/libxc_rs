//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 867/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk867<F: Float>(t6255: F, t1649: F, t1705: F, t935: F, t1656: F, t1768: F, t5740: F, t1639: F, t520: F, t5745: F, t1773: F, t522: F, t1657: F, t1772: F, t1775: F, t538: F, t5734: F, t5739: F) -> (F, F, F, F, F, F, F, F) {
    let t6256 = param_beta * t6255;
    let t6259 = t1705 * t1649;
    let t6260 = t6259 * t935;
    let t6262 = t1768 * t1656;
    let t6263 = t5740 * t6262;
    let t6267 = t1768 * t1639 * t520;
    let t6268 = t5745 * t6267;
    let t6271 = t1773 * t522 * t6255;
    let t6273 = -t1657 * t5734 - t1772 * t6271 - t1775 * t6260 + t538 * t6256 + 2.0 * t5739 * t6263 + t5739 * t6268;
    (t6256, t6259, t6260, t6262, t6263, t6268, t6271, t6273)
}
