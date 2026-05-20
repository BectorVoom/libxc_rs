//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1148/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1148<F: Float>(t12129: F, t592: F, t17: F, t184: F, t39454: F, t1287: F, t9216: F, t2223: F, t3826: F, t11985: F, t25: F, t514: F) -> (F, F, F, F, F) {
    let t39851 = t592 * t12129;
    let t39852 = F::new(48.0) * t39851;
    let t39854 = t17 * t39454 * t184;
    let t39855 = t9216 * t1287;
    let t39856 = F::new(960.0) * t39855;
    let t39857 = t2223 * t3826;
    let t39858 = F::new(384.0) * t39857;
    let t39861 = F::new(1.0) / t514 / t11985 / t25;
    (t39852, t39854, t39856, t39858, t39861)
}
