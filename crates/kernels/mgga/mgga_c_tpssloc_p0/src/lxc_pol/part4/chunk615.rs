//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 615/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk615<F: Float>(t1307: F, t550: F, t1291: F, t2663: F, t1284: F, t67: F, t758: F, t2225: F, t522: F, t2221: F, t2223: F, t2516: F, t521: F) -> (F, F, F, F, F, F, F, F) {
    let t3807 = t550 * t1307;
    let t3813 = F::cast_from(0.24415263074675393405e-3_f64) * t1291 * t2663;
    let t3814 = t1284 * t67;
    let t3815 = t3814 * t758;
    let t3819 = F::cast_from(20.0_f64) * t2225 * t522;
    let t3821 = F::cast_from(12.0_f64) * t2221 * t522;
    let t3823 = F::cast_from(32.0_f64) * t2223 * t522;
    let t3824 = t521 * t2516;
    (t3807, t3813, t3814, t3815, t3819, t3821, t3823, t3824)
}
