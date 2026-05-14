//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1176/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1176<F: Float>(t19064: F, t19186: F, t38: F, t5974: F, t1981: F, t2016: F, t55: F, t18322: F, t1985: F, t1992: F, t5971: F, t72: F, t1679: F, t5506: F, t5975: F, t18331: F, t1860: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19187 = t19064 + t19186;
    let t19191 = t38 * t5974;
    let t19192 = t1981 * t19191;
    let t19213 = t55 * t2016;
    let t19218 = 5.0 / 18.0 * t19213 * t1985 - 5.0 / 6.0 * t5971 * t1992 - t18322;
    let t19219 = t19218 * t72;
    let t19220 = t19219 * t1679;
    let t19223 = t5975 * t5506;
    let t19226 = t1860 * t18331;
    (t19187, t19191, t19192, t19213, t19218, t19219, t19220, t19223, t19226)
}
