//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1186/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1186<F: Float>(t19774: F, t5577: F, t1702: F, t226: F, t3664: F, t10579: F, t18007: F, t5572: F, t6130: F, t818: F, t782: F, t1708: F, t19724: F, t228: F, t1707: F, t17993: F, t18006: F, t19754: F, t19758: F, t19763: F, t19767: F, t19770: F, t5568: F, t5571: F, t6143: F, t6146: F) -> (F, F, F, F, F, F, F, F) {
    let t19775 = t5577 * t19774;
    let t19778 = t1702 * t3664 * t226;
    let t19779 = t5577 * t19778;
    let t19781 = t10579 * t226;
    let t19782 = t18007 * t19781;
    let t19786 = t5572 * t6130 * t818;
    let t19790 = t6130 * t782 * t226;
    let t19791 = t5577 * t19790;
    let t19794 = t1708 * t228 * t19724;
    let t19796 = -t1707 * t19794 + t17993 * t6143 - 2.0 * t18006 * t19763 + 2.0 * t19754 * t5571 + 2.0 * t19758 * t5571 - 2.0 * t19767 * t19770 + t19767 * t19782 + t19775 * t5571 + t19779 * t5571 + 2.0 * t19786 * t5571 + t19791 * t5571 - t5568 * t6146;
    (t19775, t19779, t19781, t19782, t19786, t19791, t19794, t19796)
}
