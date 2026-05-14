//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1228/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1228<F: Float>(t10584: F, t782: F, t19768: F, t1378: F, t226: F, t5562: F, t5577: F, t1702: F, t3664: F, t10579: F, t18007: F, t5572: F, t6130: F, t818: F, t1708: F, t19724: F, t228: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19769 = t10584 * t782;
    let t19770 = t19768 * t19769;
    let t19774 = t5562 * t1378 * t226;
    let t19775 = t5577 * t19774;
    let t19778 = t1702 * t3664 * t226;
    let t19779 = t5577 * t19778;
    let t19781 = t10579 * t226;
    let t19782 = t18007 * t19781;
    let t19786 = t5572 * t6130 * t818;
    let t19790 = t6130 * t782 * t226;
    let t19791 = t5577 * t19790;
    let t19794 = t1708 * t228 * t19724;
    (t19769, t19770, t19775, t19779, t19781, t19782, t19786, t19791, t19794)
}
