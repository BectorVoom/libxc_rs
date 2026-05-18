//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 182/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk182<F: Float>(t123: F, t67: F, t687: F, t3: F, t61: F, t119: F, t133: F) -> (F, F, F, F) {
    let t692 = f64::sqrt(t123);
    let t693 = t692 * t67;
    let t694 = t693 * t687;
    let t697 = F::new(1.0) / t61 / t3;
    let t698 = t119 * t697;
    let t699 = t133 * t698;
    (t693, t694, t697, t699)
}
