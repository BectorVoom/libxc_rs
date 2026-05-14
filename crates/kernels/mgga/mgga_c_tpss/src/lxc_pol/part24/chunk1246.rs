//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1246/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1246<F: Float>(t2713: F, t5608: F, t8970: F, t18092: F, t8550: F, t8557: F, t2668: F, t5600: F, t8552: F, t5605: F, t8455: F, t2738: F, t956: F, t983: F, t18067: F, t8507: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61344 = t2713 * t5608 * t8970;
    let t61350 = t8550 * t18092 * t8557;
    let t61354 = t8550 * t5608 * t8557;
    let t61363 = t5600 * t2668;
    let t61372 = t2713 * t18092 * t8970;
    let t61387 = t8550 * t8552 * sigma0 * t8557;
    let t61395 = t5605 * t8455;
    let t61422 = t983 * sigma0 * t956 * t2738;
    let t61431 = t18067 * t8507;
    (t61344, t61350, t61354, t61363, t61372, t61387, t61395, t61422, t61431)
}
