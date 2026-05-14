//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1088/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1088<F: Float>(t3216: F, t33013: F, t1068: F, t1070: F, t113633: F, t113637: F, t113655: F, t119016: F, t119065: F, t119107: F, t119149: F, t119440: F, t119485: F, t119529: F, t119571: F, t1637: F, t193: F, t23738: F, t23742: F, t25836: F, t25845: F, t30924: F, t30930: F, t336: F, t4696: F, t4700: F, t6818: F, t6822: F, t7627: F) -> (F,) {
    let t119578 = t33013 * t3216;
    let t119608 = t193 * t336 * (t119016 + t119065 + t119107 + t119149 + t119440 + t119485 + t119529 + t119571) * t1070 - t4700 * t119578 * t1068 - t4700 * t113633 * t1637 + 2.0 * t4700 * t113637 * t25845 - t4700 * t30924 * t4696 - 2.0 * t4700 * t23738 * t7627 + 4.0 * t4700 * t23742 * t7627 * t1068 - 2.0 * t4700 * t6822 * t25836 + 4.0 * t4700 * t23742 * t1637 * t6818 - 6.0 * t4700 * t113655 * t25845 + 2.0 * t4700 * t30930 * t4696;
    (t119608,)
}
