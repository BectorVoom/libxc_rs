//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1278/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1278<F: Float>(t122660: F, t2040: F, t33363: F, t7000: F, t115774: F, t1983: F, t7687: F, t1307: F, t22574: F, t26558: F, t33221: F, t12461: F, t8639: F, t26161: F, t26163: F, t119853: F, t24432: F) -> (F, F, F, F, F, F) {
    let t122662 = 2.0 * t122660 * t2040;
    let t122664 = t33363 * t7000;
    let t122667 = 3.0 * t1983 * t115774 * t7687;
    let t122671 = 6.0 * t22574 * t26558 * t33221 * t1307;
    let t122675 = t8639 * t12461;
    let t122678 = 2.0 * t26161 * t122675 * t26163;
    let t122681 = 3.0 * t22574 * t24432 * t119853;
    (t122662, t122664, t122667, t122671, t122678, t122681)
}
