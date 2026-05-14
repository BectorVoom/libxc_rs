//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 967/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk967<F: Float>(t11862: F, t4582: F, t1174: F, t11821: F, t11825: F, t11834: F, t11836: F, t11839: F, t11842: F, t11845: F, t11850: F, t11855: F, t11859: F, t1213: F, t1227: F, t1232: F, t3490: F, t3527: F, t3531: F, t3587: F, t488: F) -> (F,) {
    let t11863 = t4582 * t11862;
    let t11866 = -t11821 / 4608.0 + 5.0 / 4608.0 * t3490 * t3587 - t11825 * t1232 / 1536.0 - t3490 * t3527 / 1536.0 - t3490 * t3531 / 768.0 + t11834 + t11836 / 432.0 - t11839 / 288.0 - t11842 / 144.0 - t1174 * t11845 / 288.0 - t1174 * t11850 / 48.0 + t1213 * t11855 / 3072.0 + t11859 * t488 / 3072.0 - t1227 * t11863 / 768.0;
    (t11866,)
}
