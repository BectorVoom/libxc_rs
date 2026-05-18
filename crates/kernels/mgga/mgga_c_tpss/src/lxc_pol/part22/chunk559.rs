//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 559/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk559<F: Float>(t198: F, t207: F, t2111: F, t2114: F, t2115: F, t2116: F, t2133: F, t2224: F, t2281: F, t2285: F, t2336: F, t2340: F, t2343: F, t2351: F, t2428: F, t740: F, t823: F) -> F {
    let t2432 = t198 * t207 * t2428 * t823 + F::new(6.0) * t198 * t2115 * t2116 + F::new(3.0) * t198 * t2133 * t740 + t2111 + t2114 + t2224 - t2281 - t2285 + t2336 + t2340 - t2343 + t2351;
    t2432
}
