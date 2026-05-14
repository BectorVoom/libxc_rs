//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1100/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1100<F: Float>(t108029: F, t108649: F, t108780: F, t108856: F, t100996: F, t107571: F, t107634: F, t1401: F, t1458: F, t16524: F, t20162: F, t20347: F, t2039: F, t2098: F, t22445: F, t22448: F, t24465: F, t27254: F, t28893: F, t28951: F, t29422: F, t29425: F, t33185: F, t3941: F, t5371: F, t5456: F, t5493: F, t55388: F, t577: F, t7230: F, t75784: F, t7801: F, t7956: F, t94170: F) -> (F, F) {
    let t108858 = t108029 + t108649 + t108780 + t108856;
    let t108871 = 0.405e2 * t100996 * t1458 + 81.0 * t24465 * t22448 + 27.0 * t2098 * t22445 + 0.405e2 * t20162 * t7801 + 0.135e2 * t1401 * t107634 + 81.0 * t3941 * t28951 * t1458 + 81.0 * t3941 * t7801 * t5493 + 81.0 * t94170 * t5456 + 81.0 * t28893 * t7801 + 0.405e2 * t5371 * t28951 + 0.405e2 * t27254 * t5493 + 81.0 * t55388 * t7956 + 27.0 * t3941 * t2039 * t20347 + 81.0 * t33185 * t29425 + 0.45e1 * t108858 * t577 + 81.0 * t107571 * t2039 + 162.0 * t16524 * t29422 + 81.0 * t16524 * t29425 + 0.135e2 * t7230 * t20347 + 0.135e2 * t75784 * t2039;
    (t108858, t108871)
}
