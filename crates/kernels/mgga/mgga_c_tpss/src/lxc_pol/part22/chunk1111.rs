//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1111/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1111<F: Float>(t1707: F, t17991: F, t17993: F, t18006: F, t1809: F, t18751: F, t18753: F, t18767: F, t18771: F, t18775: F, t18779: F, t18784: F, t18789: F, t18794: F, t18797: F, t18800: F, t2408: F, t2426: F, t253: F, t5568: F, t5571: F, t5834: F, t5838: F, t5843: F, t5846: F, t819: F) -> (F,) {
    let t18802 = -t1707 * t18800 - t17991 * t1809 + 4.0 * t17993 * t5838 + 2.0 * t17993 * t5843 - 4.0 * t18006 * t18771 + t18751 * t253 - 2.0 * t18753 * t819 - 6.0 * t18767 * t5571 + 4.0 * t18775 * t5571 + 2.0 * t18779 * t5571 - 2.0 * t18784 * t5571 + 2.0 * t18789 * t5571 + t18794 * t5571 + t18797 * t5571 + 2.0 * t2408 * t5834 - t2426 * t5834 - 2.0 * t5568 * t5846;
    (t18802,)
}
