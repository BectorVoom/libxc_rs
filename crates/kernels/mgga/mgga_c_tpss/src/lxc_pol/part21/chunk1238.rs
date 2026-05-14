//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1238/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1238<F: Float>(t2697: F, t65: F, t3749: F, t928: F, t3754: F, t3941: F, t5610: F, t1461: F, t1467: F, t18069: F, t18076: F, t18079: F, t18094: F, t18107: F, t18110: F, t18113: F, t19847: F, t19849: F, t3928: F, t3935: F, t3945: F, t5605: F) -> (F, F, F, F, F) {
    let t19850 = t65 * t2697;
    let t19851 = t19850 * t3749;
    let t19854 = t65 * t928;
    let t19855 = t19854 * t3754;
    let t19864 = t5610 * t3941;
    let t19868 = -t18076 / 108.0 - t18079 + t18113 / 864.0 - t18110 * t1461 / 108.0 + t19847 / 864.0 + t19849 * t19851 / 216.0 - t19849 * t19855 / 144.0 + t5605 * t3928 / 288.0 + t18094 * t3935 / 768.0 - t18107 * t1467 / 288.0 + t19864 / 2304.0 + t18069 * t3945 / 2304.0;
    (t19850, t19851, t19854, t19855, t19868)
}
