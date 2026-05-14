//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 748/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk748<F: Float>(t3948: F, t3986: F, t1475: F, t219: F, t1482: F, t990: F, t2776: F, t2786: F, t948: F, t1464: F, t975: F, t366: F, t3949: F, t2785: F, t345: F, t1474: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3987 = t3948 + t3986;
    let t3988 = param_beta * t3987;
    let t3990 = t1475 * t219;
    let t3993 = t1482 * t990;
    let t3994 = t2776 * t3993;
    let t3997 = t2786 * t948;
    let t4001 = t975 * t1464;
    let t4004 = t366 * t3949;
    let t4008 = t2785 * t948 * t345;
    let t4011 = t1474 * t948;
    (t3987, t3988, t3990, t3994, t3997, t4001, t4004, t4008, t4011)
}
