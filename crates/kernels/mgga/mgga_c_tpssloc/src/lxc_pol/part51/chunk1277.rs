//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1277/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1277<F: Float>(t22716: F, t8480: F, t31203: F, t6914: F, t31207: F, t6883: F, t22724: F, t31198: F, t22704: F, t22705: F, t31202: F, t22674: F, t31123: F, t6897: F) -> (F, F, F, F, F, F) {
    let t114104 = F::cast_from(0.12793931631041761173e0_f64) * t22716 * t8480;
    let t114105 = t6914 * t31203;
    let t114116 = t6883 * t31207;
    let t114119 = F::cast_from(0.52089578783527170489e-1_f64) * t22724 * t31198;
    let t114121 = t22704 * t22705 * t31202;
    let t114154 = t6897 * t22674 * t31123;
    (t114104, t114105, t114116, t114119, t114121, t114154)
}
