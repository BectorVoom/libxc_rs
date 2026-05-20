//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 754/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk754<F: Float>(t28017: F, t510: F, t652: F, t7685: F, t7756: F, t5493: F, t89: F) -> (F, F, F, F) {
    let t28025 = t510 * t28017;
    let t28027 = F::new(2.0) * t652 * t28025;
    let t28029 = F::new(2.0) * t7685 * t7756;
    let t28030 = t89 * t5493;
    (t28025, t28027, t28029, t28030)
}
