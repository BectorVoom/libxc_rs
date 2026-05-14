//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1266/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1266<F: Float>(t5608: F, t8550: F, t8557: F, t18073: F, t921: F, t2668: F, t5600: F, t5620: F, t9003: F, t8956: F, t938: F, t18092: F, t2713: F, t8970: F, t5605: F, t8483: F) -> (F, F, F, F, F, F, F) {
    let t61354 = t8550 * t5608 * t8557;
    let t61361 = t18073 * t921;
    let t61363 = t5600 * t2668;
    let t61365 = t5620 * t9003;
    let t61368 = t938 * t5608 * t8956;
    let t61372 = t2713 * t18092 * t8970;
    let t61377 = t5605 * t8483;
    (t61354, t61361, t61363, t61365, t61368, t61372, t61377)
}
