//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1378/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1378<F: Float>(t19345: F, t5790: F, t18350: F, t20275: F, t5492: F, t18338: F, t18360: F, t18366: F, t18666: F, t19342: F, t19349: F, t20246: F, t20264: F, t20282: F, t62007: F, t62277: F, t62356: F, t6304: F, t65152: F, t65162: F, t65202: F) -> F {
    let t67472 = t5790 * t19345;
    let t67474 = F::new(160.0) / F::new(9.0) * t18350 * t67472;
    let t67480 = F::new(32.0) / F::new(9.0) * t5492 * t20275;
    let t67489 = F::new(20.0) * t62277 * t19342 + F::new(20.0) * t18666 * t65202 + F::new(20.0) * t18666 * t65162 + F::new(10.0) * t18666 * t65152 - t67474 + F::new(20.0) / F::new(3.0) * t62007 * t20264 + F::new(10.0) / F::new(3.0) * t19349 * t62356 + t67480 - F::new(4.0) / F::new(3.0) * t18338 * t6304 - F::new(2.0) / F::new(3.0) * t18366 * t6304 - F::new(4.0) / F::new(3.0) * t5492 * t20282 - F::new(5.0) / F::new(3.0) * t20246 * t18360;
    t67489
}
