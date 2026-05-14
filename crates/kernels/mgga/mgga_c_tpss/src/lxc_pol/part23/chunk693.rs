//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 693/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk693<F: Float>(t2349: F, t3308: F, t187: F, t3297: F, t3180: F, t3182: F, t3192: F, t3194: F, t3196: F, t3213: F, t3216: F, t3302: F, t3304: F, t3307: F, t219: F, t3300: F) -> (F, F, F) {
    let t3310 = 0.10843581300301739842e-1 * t3308 * t2349;
    let t3312 = 0.19751673498613801407e-1 * t3297 * t187;
    let t3313 = t3302 + t3304 + t3307 + t3213 - t3216 + t3310 + t3312 - t3192 + t3194 - t3196 - t3180 - t3182;
    let t3315 = (t3300 + t3313) * t219;
    (t3310, t3312, t3315)
}
