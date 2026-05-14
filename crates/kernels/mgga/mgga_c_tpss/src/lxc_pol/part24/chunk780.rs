//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 780/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk780<F: Float>(t1224: F, t5381: F, t774: F, t1625: F, t520: F, t4416: F, t3273: F, t2281: F, t2285: F, t2292: F, t2302: F, t2310: F, t3189: F, t3209: F, t3281: F, t3304: F, t5347: F, t5348: F) -> (F, F, F, F) {
    let t5383 = t1224 * t774 * t5381;
    let t5387 = t520 * t1625;
    let t5388 = t4416 * t5387;
    let t5389 = t3273 * t5388;
    let t5392 = t2302 + t2310 - t2292 - t2281 - t2285 + t3281 - t3209 - t5348 - t5347 + t3189 - t3304;
    (t5383, t5387, t5389, t5392)
}
