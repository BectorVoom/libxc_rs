//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1250/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1250<F: Float>(t228: F, t32386: F, t18005: F, t5567: F, t1706: F, t5570: F, t8347: F, t5562: F, t768: F, t18042: F, t2436: F, t198: F, t206: F, t5585: F, t33457: F, t33459: F) -> (F, F, F, F, F, F, F) {
    let t61195 = t32386 * t228;
    let t61222 = t5567 * t18005;
    let t61226 = t1706 * t5570 * t8347;
    let t61232 = t768 * t5562;
    let t61264 = t18042 * t2436;
    let t61269 = t198 * t206 * t5585;
    let t61283 = t33457 * t33459;
    (t61195, t61222, t61226, t61232, t61264, t61269, t61283)
}
