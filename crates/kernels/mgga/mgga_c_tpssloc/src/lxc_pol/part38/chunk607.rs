//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 607/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk607<F: Float>(t2988: F, t2990: F, t2775: F, t344: F, t2244: F, t977: F, t2250: F, t978: F, t2822: F, t2824: F, t2828: F, t2831: F, t2834: F, t340: F, t343: F, t974: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2991 = t2988 * t2990;
    let t2994 = t344 * t2775;
    let t2995 = t2994 * t2244;
    let t2996 = t977 * t2995;
    let t2999 = t978 * t2250;
    let t3000 = t977 * t2999;
    let t3003 = 5.0 / 18.0 * t2822;
    let t3008 = -t3003 - 2.0 / 9.0 * t2824 + t2828 / 18.0 - t2831 / 3.0 + t2834 / 6.0;
    let t3009 = t340 * t3008;
    let t3010 = t3009 * t343;
    let t3011 = t974 * t3010;
    (t2991, t2995, t2996, t2999, t3000, t3003, t3008, t3010, t3011)
}
