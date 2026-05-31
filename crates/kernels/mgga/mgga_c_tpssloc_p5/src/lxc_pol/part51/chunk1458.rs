//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1458/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1458<F: Float>(t12461: F, t8639: F, t26161: F, t26163: F, t119853: F, t22574: F, t24432: F, t671: F, t8518: F, t1983: F, t31035: F, t7940: F) -> (F, F, F, F) {
    let t122675 = t8639 * t12461;
    let t122678 = F::cast_from(2.0_f64) * t26161 * t122675 * t26163;
    let t122681 = F::cast_from(3.0_f64) * t22574 * t24432 * t119853;
    let t122685 = t8518 * t671;
    let t122692 = t1983 * t7940 * t31035;
    (t122678, t122681, t122685, t122692)
}
