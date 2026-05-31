//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 342/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk342<F: Float>(t1112: F, t1114: F, t242: F, t127: F, t359: F, t461: F, t460: F, t357: F, t458: F, t339: F, t454: F) -> (F, F, F, F) {
    let t1115 = t1112 * t1114;
    let t1116 = t242 * t1115;
    let t1120 = t359 * t127 * t461;
    let t1122 = t460 * t1120 / F::cast_from(4608.0_f64);
    let t1123 = t458 * t357;
    let t1125 = t339 * t454 * t1123;
    (t1116, t1120, t1122, t1125)
}
