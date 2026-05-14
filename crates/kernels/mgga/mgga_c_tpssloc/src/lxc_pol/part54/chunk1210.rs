//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1210/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1210<F: Float>(t33603: F, t6876: F, t31297: F, t7685: F, t191: F, t192: F, t27215: F, t2020: F, t26142: F, t7042: F, t25010: F, t8607: F, t23938: F, t7468: F, t26977: F, t26003: F) -> (F, F, F, F, F, F, F, F) {
    let t121203 = 3.0 * t6876 * t33603;
    let t121204 = t7685 * t31297;
    let t121210 = t27215 * t191 * t192;
    let t121211 = t121210 * t2020;
    let t121224 = 2.0 * t7042 * t26142;
    let t121226 = t8607 * t25010;
    let t121228 = 2.0 * t23938 * t7468;
    let t121231 = 2.0 * t26977 * t7468;
    let t121233 = 2.0 * t7042 * t26003;
    (t121203, t121204, t121211, t121224, t121226, t121228, t121231, t121233)
}
