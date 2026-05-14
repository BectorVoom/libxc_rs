//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1346/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1346<F: Float>(t10464: F, t2105: F, t5986: F, t626: F, t6540: F, t65902: F, t65904: F, t65906: F, t65908: F, t65910: F, t65915: F, t65917: F, t65921: F, t65923: F, t65927: F, t65929: F, t65931: F, t65933: F, t65935: F, t65937: F, t65942: F, t65945: F) -> (F,) {
    let t68701 = -2.0 * t2105 * t626 * t6540 - 2.0 * t10464 * t5986 - t65902 - t65904 - t65906 - t65908 - t65910 - t65915 - t65917 - t65921 - t65923 - t65927 - t65929 - t65931 - t65933 - t65935 + t65937 + t65942 + t65945;
    (t68701,)
}
