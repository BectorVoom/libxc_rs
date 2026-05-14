//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 633/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk633<F: Float>(t2761: F, t66: F, t2460: F, t242: F, t2690: F, t2693: F, t2700: F, t2706: F, t2722: F, t2727: F, t2731: F, t2734: F, t2740: F, t2743: F, t2748: F, t2754: F, t2757: F, t925: F, t946: F, t967: F, t972: F) -> (F, F, F) {
    let t2762 = t66 * t2761;
    let t2763 = t2762 * t2460;
    let t2764 = t242 * t2763;
    let t2767 = t2690 / 432.0 + t925 * t2693 / 288.0 + t925 * t2700 / 216.0 + t946 * t2706 / 3072.0 + t2722 * t2727 / 1536.0 - t2731 * t2734 / 3072.0 + t2740 * t2743 / 2304.0 - t2748 * t972 / 432.0 + t2754 / 3456.0 + t967 * t2757 / 4608.0 + 5.0 / 13824.0 * t967 * t2764;
    (t2762, t2764, t2767)
}
