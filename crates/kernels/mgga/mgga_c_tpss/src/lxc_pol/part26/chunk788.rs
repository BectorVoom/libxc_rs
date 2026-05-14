//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 788/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk788<F: Float>(t189: F, t5343: F, t489: F, t3182: F, t3194: F, t3196: F, t3213: F, t3216: F, t3307: F, t3310: F, t5326: F, t5327: F, t5345: F, t5346: F, t219: F, t5392: F) -> (F, F, F) {
    let t5393 = t5343 * t189;
    let t5394 = t489 * t5393;
    let t5395 = t5394 + t5345 + t3307 + t3213 + t3216 + t5346 + t3310 - t5326 - t5327 + t3194 - t3196 - t3182;
    let t5397 = (t5392 + t5395) * t219;
    (t5393, t5394, t5397)
}
