//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1000/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1000<F: Float>(t121349: F, t1527: F, t1888: F, t23270: F, t1880: F, t214: F, t225: F, t258: F, t29040: F, t118578: F, t118580: F, t123566: F, t123571: F, t123572: F, t126294: F, t126298: F, t126302: F, t126306: F, t126309: F, t126312: F, t126316: F, t126320: F) -> (F, F, F) {
    let t127889 = t1888 * t23270 * t121349 * t1527;
    let t127896 = t1880 * t214 * t29040 * t225 * t258;
    let t127908 = -t126294 / F::new(384.0) - t126298 / F::new(768.0) + t126302 / F::new(384.0) - t126306 / F::new(768.0) - F::new(0.16149102437656156341e-2) * t126309 + t123566 + t126312 / F::new(96.0) + F::new(0.22608743412718618878e-1) * t118578 + F::new(0.13565246047631171327e0) * t118580 + t126316 / F::new(768.0) - F::new(0.96894614625936938046e-2) * t126320 + t123571 + t123572;
    (t127889, t127896, t127908)
}
