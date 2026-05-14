//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1267/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1267<F: Float>(t8550: F, t8552: F, t8557: F, t18072: F, t762: F, t18110: F, t2689: F, t5605: F, t8455: F, t5620: F, t8530: F, t1721: F, t339: F, t8961: F, t18094: F, t8434: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t61387 = t8550 * t8552 * sigma0 * t8557;
    let t61390 = t18072 * t762;
    let t61393 = t18110 * t2689;
    let t61395 = t5605 * t8455;
    let t61401 = t5620 * t8530;
    let t61406 = t339 * t1721 * t8961;
    let t61409 = t18094 * t8434;
    (t61387, t61390, t61393, t61395, t61401, t61406, t61409)
}
