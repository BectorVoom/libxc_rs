//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1269/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1269<F: Float>(t20446: F, t219: F, t6338: F, t18000: F, t6342: F, t818: F, t18770: F, t19748: F, t1396: F, t17993: F, t18006: F, t1809: F, t18753: F, t19734: F, t19736: F, t253: F, t3699: F, t3722: F, t5571: F, t5834: F, t5838: F, t5843: F, t5846: F, t6135: F, t6343: F, t819: F, param_beta: F) -> (F, F, F, F, F) {
    let t20447 = param_beta * t20446;
    let t20449 = t6338 * t219;
    let t20463 = t18000 * t6342 * t818;
    let t20466 = t18770 * t19748;
    let t20469 = -t1396 * t18753 + F::new(2.0) * t17993 * t6343 - F::new(2.0) * t18006 * t20466 - t1809 * t19734 + F::new(2.0) * t19736 * t5838 + t19736 * t5843 + t20447 * t253 - t20449 * t819 - F::new(6.0) * t20463 * t5571 + F::new(2.0) * t3699 * t5834 - t3722 * t5834 - t5846 * t6135;
    (t20447, t20449, t20463, t20466, t20469)
}
