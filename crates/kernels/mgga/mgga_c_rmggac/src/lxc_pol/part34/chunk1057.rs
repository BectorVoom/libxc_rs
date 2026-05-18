//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1057/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1057<F: Float>(t78131: F, t78146: F, t78164: F, t78182: F, t15536: F, t40826: F, t72062: F, t14451: F, t1614: F, t4669: F, t72020: F, t8636: F) -> (F, F, F, F, F) {
    let t78184 = t78131 + t78146 + t78164 + t78182;
    let t78188 = t40826 * t15536;
    let t78189 = F::new(0.2993560425465952141e-1) * t78188;
    let t78194 = F::new(0.90915538847484472429e-2) * t72062;
    let t78198 = t4669 * t14451 * t1614;
    let t78199 = F::new(0.44903406381989282115e-1) * t78198;
    let t78200 = t72020 * t8636;
    (t78184, t78189, t78194, t78199, t78200)
}
