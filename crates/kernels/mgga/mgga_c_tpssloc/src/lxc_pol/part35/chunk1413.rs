//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1413/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1413<F: Float>(t22685: F, t26193: F, t28191: F, t1985: F, t28232: F, t107250: F, t107260: F, t20060: F, t2016: F, t20662: F, t26366: F, t28187: F, t28220: F, t5215: F, t5321: F, t6461: F, t6958: F, t74849: F, t7750: F, t81318: F) -> F {
    let t107265 = t22685 * t26193 * t28191;
    let t107268 = t1985 * t26193 * t28232;
    let t107270 = -t81318 - t74849 * t2016 + F::new(12.0) * t5321 * t28220 - F::new(0.49348022005446793095e-1) * t107250 - F::new(3.0) * t26366 * t6461 - F::new(3.0) * t20060 * t7750 - t6958 * t20662 + F::new(0.82246703342411321825e-2) * t107260 - F::new(3.0) * t5215 * t28187 + F::new(0.14804406601634037928e0) * t107265 + F::new(0.49348022005446793095e-1) * t107268;
    t107270
}
