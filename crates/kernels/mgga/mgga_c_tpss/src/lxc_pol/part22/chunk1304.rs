//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1304/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1304<F: Float>(t61079: F, t18751: F, t219: F, t1811: F, t31814: F, t18802: F, t2436: F, t5848: F, t8096: F, t61868: F, t18999: F, t508: F) -> (F, F, F, F, F, F, F) {
    let t62711 = F::new(455.0) / F::new(648.0) * t61079;
    let t62731 = t18751 * t219;
    let t62807 = t1811 * t31814;
    let t62820 = t18802 * t2436;
    let t62829 = t5848 * t8096;
    let t63006 = F::new(308.0) / F::new(27.0) * t61868;
    let t63101 = t508 * t18999;
    (t62711, t62731, t62807, t62820, t62829, t63006, t63101)
}
