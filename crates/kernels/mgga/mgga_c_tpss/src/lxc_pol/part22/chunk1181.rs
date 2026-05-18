//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1181/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1181<F: Float>(t1279: F, t1281: F, t13265: F, t13279: F, t13283: F, t13286: F, t13289: F, t1668: F, t1670: F, t3403: F, t3407: F, t3410: F, t4549: F, t4556: F, t4559: F, t547: F, t548: F) -> F {
    let t13292 = F::new(12.0) * t1279 * t4556 + F::new(6.0) * t1279 * t4559 + F::new(6.0) * t1281 * t4549 + t13265 * t548 + F::new(6.0) * t13279 * t547 + F::new(12.0) * t13283 * t547 + F::new(6.0) * t13286 * t547 + F::new(3.0) * t13289 * t547 + F::new(6.0) * t1668 * t3407 + F::new(3.0) * t1668 * t3410 + F::new(3.0) * t1670 * t3403;
    t13292
}
