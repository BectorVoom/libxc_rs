//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1206/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1206<F: Float>(t1707: F, t17993: F, t18006: F, t19754: F, t19758: F, t19763: F, t19767: F, t19770: F, t19775: F, t19779: F, t19782: F, t19786: F, t19791: F, t19794: F, t5568: F, t5571: F, t6143: F, t6146: F) -> (F,) {
    let t19796 = -t1707 * t19794 + t17993 * t6143 - 2.0 * t18006 * t19763 + 2.0 * t19754 * t5571 + 2.0 * t19758 * t5571 - 2.0 * t19767 * t19770 + t19767 * t19782 + t19775 * t5571 + t19779 * t5571 + 2.0 * t19786 * t5571 + t19791 * t5571 - t5568 * t6146;
    (t19796,)
}
