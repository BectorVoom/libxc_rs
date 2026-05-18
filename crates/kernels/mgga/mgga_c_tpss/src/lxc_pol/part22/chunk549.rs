//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 549/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk549<F: Float>(t2348: F, t2349: F, t2111: F, t2114: F, t2208: F, t2217: F, t2220: F, t2224: F, t2242: F, t2244: F, t2246: F, t2281: F, t2285: F, t2292: F, t2302: F, t2310: F, t2333: F, t2336: F, t2340: F, t2343: F, t2347: F) -> (F, F) {
    let t2351 = F::new(0.10843581300301739842e-1) * t2348 * t2349;
    let t2352 = -t2208 - t2217 - t2220 + t2224 + t2242 + t2244 + t2246 + t2333 + t2302 + t2310 + t2111 + t2114 + t2336 - t2292 + t2340 - t2281 - t2343 + t2347 - t2285 + t2351;
    (t2351, t2352)
}
