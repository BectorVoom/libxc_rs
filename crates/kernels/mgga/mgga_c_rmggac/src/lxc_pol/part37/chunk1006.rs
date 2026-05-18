//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1006/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1006<F: Float>(t15536: F, t40826: F, t72062: F, t14451: F, t1614: F, t4669: F, t72020: F, t8636: F, t72023: F, t8902: F, t8906: F, t22: F, t326: F, t8041: F) -> (F, F, F, F, F, F, F) {
    let t78188 = t40826 * t15536;
    let t78189 = F::new(0.2993560425465952141e-1) * t78188;
    let t78194 = F::new(0.90915538847484472429e-2) * t72062;
    let t78198 = t4669 * t14451 * t1614;
    let t78199 = F::new(0.44903406381989282115e-1) * t78198;
    let t78200 = t72020 * t8636;
    let t78201 = F::new(0.27274661654245341728e-1) * t78200;
    let t78202 = t72023 * t8902;
    let t78203 = F::new(0.20455996240684006297e-1) * t78202;
    let t78204 = t72020 * t8906;
    let t78205 = F::new(0.27274661654245341729e-1) * t78204;
    let t78207 = t326 * t8041 * t22;
    (t78189, t78194, t78199, t78201, t78203, t78205, t78207)
}
