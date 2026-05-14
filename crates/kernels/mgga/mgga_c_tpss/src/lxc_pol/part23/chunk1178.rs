//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1178/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1178<F: Float>(t5: F, t19238: F, t117: F, t1864: F, t2061: F, t6054: F, t645: F, t1163: F, t118: F, t17900: F, t17902: F, t17904: F, t17906: F, t17909: F, t17911: F, t17913: F, t17915: F, t18292: F, t18298: F, t18304: F, t18384: F, t18386: F, t1865: F, t19187: F, t3166: F, t485: F, t5984: F, t626: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t19239 = piecewise3(t8, 0.0, t19238);
    let t19240 = t19239 * t117;
    let t19244 = t1864 * t2061;
    let t19247 = t6054 * t645;
    let t19250 = -2.0 * t1163 * t5984 - t118 * t19187 - t1865 * t3166 - t19240 * t485 - 2.0 * t19244 * t485 - 4.0 * t19247 * t626 - t17900 - t17902 + t17904 - t17906 - t17909 - t17911 - t17913 - t17915 + t18292 + t18298 - t18304 - t18384 - t18386;
    (t19239, t19240, t19244, t19247, t19250)
}
