//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 809/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk809<F: Float>(t4028: F, t8327: F, t7458: F, t1774: F, t8326: F, t652: F, t1799: F, t1998: F, t59: F, t6926: F, t1825: F, t6943: F, t6936: F, t1814: F, t8465: F, t8467: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32673 = t4028 * t8327;
    let t32674 = 2.0 * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = 2.0 * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = 2.0 * t32678;
    let t32711 = t1998 * t59 * t1799;
    let t32712 = t6926 * t32711;
    let t32714 = t6943 * t1825;
    let t32715 = t6936 * t32714;
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    (t32674, t32676, t32677, t32679, t32711, t32712, t32714, t32715, t32717, t32718)
}
