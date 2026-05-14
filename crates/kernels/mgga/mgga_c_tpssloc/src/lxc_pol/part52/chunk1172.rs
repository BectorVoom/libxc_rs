//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1172/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1172<F: Float>(t119853: F, t22574: F, t8643: F, t31048: F, t7685: F, t31033: F, t1983: F, t33136: F, t6996: F, t652: F, t6862: F, t7467: F, t2314: F, t32670: F, t32782: F, t6999: F) -> (F, F, F, F, F, F, F) {
    let t119856 = 6.0 * t22574 * t8643 * t119853;
    let t119858 = 3.0 * t7685 * t31048;
    let t119862 = t7685 * t31033;
    let t119867 = 2.0 * t1983 * t6996 * t33136;
    let t119869 = t652 * t6862 * t7467;
    let t119871 = t2314 * t32670;
    let t119874 = t1983 * t32782 * t6999;
    (t119856, t119858, t119862, t119867, t119869, t119871, t119874)
}
