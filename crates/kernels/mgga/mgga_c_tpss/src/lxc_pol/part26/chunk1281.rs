//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1281/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1281<F: Float>(t114: F, t68887: F, t485: F, t626: F, t19579: F, t19580: F, t51664: F, t21253: F, t5710: F, t19602: F, t6243: F, t21175: F, t5706: F, t21180: F, t5532: F, t13133: F, t6106: F) -> (F, F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t68888 = piecewise3(t115, 0.0, t68887);
    let t68891 = 2.0 * t626 * t485 * t68888;
    let t68905 = 2.0 * t19579 * t19580 * t51664;
    let t68907 = 3.0 * t21253 * t5710;
    let t68909 = 2.0 * t6243 * t19602;
    let t68913 = 2.0 * t5706 * t21175;
    let t68915 = 4.0 * t21180 * t5532;
    let t68917 = 4.0 * t13133 * t6106;
    (t68888, t68891, t68905, t68907, t68909, t68913, t68915, t68917)
}
