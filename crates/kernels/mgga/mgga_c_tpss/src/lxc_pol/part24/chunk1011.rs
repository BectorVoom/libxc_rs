//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1011/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1011<F: Float>(t14245: F, t2389: F, t774: F, t10617: F, t10620: F, t10630: F, t10635: F, t10642: F, t14220: F, t14223: F, t14229: F, t14234: F, t14238: F, t14242: F, t2173: F, t3626: F, t797: F, t8131: F) -> (F, F) {
    let t14247 = t2389 * t774 * t14245;
    let t14250 = -7.0 / 576.0 * t14220 + t3626 * t14223 / 1536.0 - 119.0 / 1728.0 * t10617 + t10620 - 119.0 / 3456.0 * t8131 + t2173 * t14229 / 384.0 + t2173 * t14234 / 384.0 + t10630 - 35.0 / 108.0 * t10635 - t10642 + 7.0 / 4608.0 * t14238 - 5.0 / 128.0 * t797 * t14242 + 5.0 / 384.0 * t797 * t14247;
    (t14247, t14250)
}
