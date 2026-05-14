//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1259/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1259<F: Float>(t42181: F, t5486: F, t19424: F, t7682: F, t19367: F, t1981: F, t38: F, t1333: F, t61870: F, t19590: F, t61873: F, t18394: F, t3532: F, t116: F, t19430: F, t18546: F, t6242: F) -> (F, F, F, F, F, F, F, F) {
    let t65406 = t42181 * t5486;
    let t65413 = t7682 * t19424;
    let t65417 = t1981 * t38 * t19367;
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    let t65443 = 4.0 / 3.0 * t65442;
    let t65444 = t18394 * t3532;
    let t65445 = 2.0 / 3.0 * t65444;
    let t65490 = t19430 * t116;
    let t65533 = t6242 * t18546;
    (t65406, t65413, t65417, t65440, t65443, t65445, t65490, t65533)
}
