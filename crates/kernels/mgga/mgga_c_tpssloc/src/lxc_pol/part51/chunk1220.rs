//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1220/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1220<F: Float>(t26149: F, t8607: F, t26161: F, t33221: F, t92200: F, t1388: F, t92169: F, t120016: F, t1983: F, t2095: F, t31669: F, t5161: F, t25992: F, t102344: F, t1874: F, t27188: F, t6525: F) -> (F, F, F, F, F, F, F, F) {
    let t121181 = t8607 * t26149;
    let t121184 = 2.0 * t26161 * t92200 * t33221;
    let t121190 = 6.0 * t26161 * t92169 * t33221 * t1388;
    let t121192 = t1983 * t2095 * t120016;
    let t121194 = t1983 * t31669 * t5161;
    let t121195 = t8607 * t25992;
    let t121197 = 2.0 * t102344 * t1874;
    let t121199 = 2.0 * t27188 * t6525;
    (t121181, t121184, t121190, t121192, t121194, t121195, t121197, t121199)
}
