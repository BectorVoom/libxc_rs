//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 714/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk714<F: Float>(t72: F, t9338: F, t2245: F, t2252: F, t2255: F, t2284: F, t2304: F, t609: F, t629: F, t642: F, t66: F, t80: F, t9247: F, t9248: F, t9251: F, t9260: F, t9263: F, t9268: F, t9313: F) -> F {
    let t9339 = t72 * t9338;
    let t9342 = -t9247 * t9248 / F::new(4.0) - t9251 * t80 / F::new(4.0) - t2245 * t642 / F::new(4.0) - t9260 * t80 / F::new(12.0) - t9263 * t80 / F::new(4.0) - t2252 * t642 / F::new(4.0) - t9268 * t80 / F::new(4.0) - t2255 * t642 / F::new(2.0) - t609 * t2304 / F::new(4.0) + t9313 * t80 / F::new(24.0) + t2284 * t642 / F::new(8.0) + t629 * t2304 / F::new(8.0) + t66 * t9339 / F::new(24.0);
    t9342
}
