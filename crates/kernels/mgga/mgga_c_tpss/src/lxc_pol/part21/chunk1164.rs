//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1164/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1164<F: Float>(t18069: F, t18073: F, t18076: F, t18079: F, t18083: F, t18086: F, t18094: F, t18098: F, t2706: F, t2727: F, t2734: F, t2743: F, t2757: F, t2764: F, t336: F, t5610: F, t5620: F, t972: F) -> (F,) {
    let t18101 = t18069 * t2743 / 1152.0 + 11.0 / 108.0 * t18073 * t336 - t18076 / 54.0 - t18079 + 5.0 / 6912.0 * t5620 * t2764 - t18083 * t972 / 216.0 + t18086 / 1728.0 + t5620 * t2757 / 2304.0 + t5610 * t2706 / 1536.0 + t18094 * t2727 / 768.0 - t18098 * t2734 / 1536.0;
    (t18101,)
}
