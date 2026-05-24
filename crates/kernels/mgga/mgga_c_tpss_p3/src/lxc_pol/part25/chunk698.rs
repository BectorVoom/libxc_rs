//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 698/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk698<F: Float>(t1219: F, t1649: F, t1233: F, t1260: F, t1640: F, t220: F, t3374: F, t339: F, t4417: F, t4460: F, t4487: F, t4498: F, t4499: F, t4508: F, t523: F) -> (F, F) {
    let t4511 = t1219 * t1649;
    let t4516 = -t1233 * t339 * t4511 - t1233 * t4499 * t4508 - t1260 * t339 * t4460 - t1640 * t3374 * t339 + t220 * t4487 * t523 + F::new(2.0) * t4417 * t4498 * t4499;
    (t4511, t4516)
}
