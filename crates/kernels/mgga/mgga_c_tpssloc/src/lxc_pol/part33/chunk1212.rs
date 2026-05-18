//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1212/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1212<F: Float>(t21669: F, t225: F, t21684: F, t20675: F, t3701: F, t20684: F, t40611: F, t20602: F, t20420: F, t20672: F, t20670: F, t1834: F, t6414: F) -> (F, F, F, F, F, F, F, F, F) {
    let t70980 = t21669 * t225;
    let t70987 = t21684 * t225;
    let t74014 = t20675 * t3701;
    let t74064 = t20684 * t40611;
    let t74849 = t20602 * t225;
    let t74860 = t20420 * t225;
    let t74908 = t20672 * t225;
    let t74930 = t20670 * t225;
    let t74937 = t1834 * t6414;
    (t70980, t70987, t74014, t74064, t74849, t74860, t74908, t74930, t74937)
}
