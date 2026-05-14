//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1070/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1070<F: Float>(t118940: F, t1880: F, t25224: F, t30656: F, t113038: F, t113045: F, t118916: F, t118917: F, t118918: F, t118924: F, t118928: F, t118935: F, t118938: F, t23281: F, t25188: F, t25233: F, t25330: F, t2597: F, t32853: F, t6627: F, t6663: F, t7538: F) -> (F,) {
    let t118941 = 0.16449340668482264365e-1 * t118940;
    let t118944 = 0.16449340668482264365e-1 * t1880 * t25224 * t30656;
    let t118945 = -2.0 * t23281 * t7538 - 2.0 * t25188 * t6663 + 4.0 * t25233 * t6627 - 2.0 * t25330 * t6627 - t2597 * t32853 + t113038 - t113045 + t118916 + t118917 + t118918 - t118924 + t118928 - t118935 - t118938 + t118941 - t118944;
    (t118945,)
}
