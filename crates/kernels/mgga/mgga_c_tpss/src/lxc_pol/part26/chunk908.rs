//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 908/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk908<F: Float>(t2215: F, t651: F, t2348: F, t123: F, t727: F, t2349: F, t2192: F, t737: F, t2345: F, t767: F, t230: F, t2162: F, t226: F, t2376: F, t339: F, t769: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8223 = t651 * t2215;
    let t8225 = 0.48159733137676571078e0 * t2348 * t8223;
    let t8226 = t727 * t123;
    let t8227 = t8226 * t2349;
    let t8229 = t2192 * t737;
    let t8231 = 0.21687162600603479684e-1 * t2348 * t8229;
    let t8232 = t651 * t2345;
    let t8234 = 0.32530743900905219526e-1 * t2348 * t8232;
    let t8274 = t767 * t767;
    let t8275 = 1.0 / t8274;
    let t8276 = t8275 * t230;
    let t8279 = t2162 * t226;
    let t8286 = t339 * t769 * t2376;
    (t8223, t8225, t8227, t8229, t8231, t8232, t8234, t8275, t8276, t8279, t8286)
}
