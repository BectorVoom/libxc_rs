//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 768/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk768<F: Float>(t334: F, t371: F, t533: F, t556: F, t1433: F, t71: F, t1458: F, t89: F, t1597: F, t343: F, t88: F, t2130: F, rho1: F) -> (F, F, F, F, F, F, F) {
    let t6793 = t371 * t334;
    let t6924 = F::cast_from(1.0_f64) / t556 / t533;
    let t7445 = t71 * t1433;
    let t7458 = t89 * t1458;
    let t7577 = t1597 * t343;
    let t7676 = t88 * t1458;
    let t8025 = t2130 * rho1;
    (t6793, t6924, t7445, t7458, t7577, t7676, t8025)
}
