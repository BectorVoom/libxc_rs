//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1345/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1345<F: Float>(t12825: F, t18454: F, t236: F, t339: F, t60698: F, t12894: F, t12836: F, t19469: F, t215: F, t12841: F, t18464: F, t4480: F, t12838: F, t5728: F, t12843: F, t12877: F) -> (F, F, F, F, F, F, F, F) {
    let t65604 = t18454 * t12825;
    let t65607 = t339 * t60698 * t236;
    let t65608 = t65607 * t12894;
    let t65611 = t19469 * t215 * t12836;
    let t65614 = t19469 * t215 * t12841;
    let t65616 = t18464 * t4480;
    let t65617 = 35.0 / 288.0 * t65616;
    let t65618 = t5728 * t12838;
    let t65620 = t5728 * t12843;
    let t65622 = t18454 * t12877;
    (t65604, t65608, t65611, t65614, t65617, t65618, t65620, t65622)
}
