//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1210/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1210<F: Float>(t2239: F, t5385: F, t1597: F, t976: F, t3131: F, t5866: F, t111: F, t20292: F, t21038: F, t225: F, t21061: F, t21036: F) -> (F, F, F, F, F, F, F) {
    let t55921 = t5385 * t2239;
    let t61066 = t976 * t1597;
    let t62840 = t5866 * t3131;
    let t67001 = t20292 * t111;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    (t55921, t61066, t62840, t67001, t67305, t67339, t67344)
}
