//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1135/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1135<F: Float>(t43705: F, t11604: F, t496: F, t68: F, t1406: F, t9238: F, t111: F, t6470: F, t2239: F, t5385: F, t1176: F, t1714: F, t20292: F, t21038: F, t225: F, t21061: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43706 = 1.0 / t43705;
    let t45349 = 1.0 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45844 = t1406 * t9238;
    let t55388 = t6470 * t111;
    let t55921 = t5385 * t2239;
    let t64825 = t1176 * t1714;
    let t67001 = t20292 * t111;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    (t43706, t45350, t45844, t55388, t55921, t64825, t67001, t67305, t67339)
}
