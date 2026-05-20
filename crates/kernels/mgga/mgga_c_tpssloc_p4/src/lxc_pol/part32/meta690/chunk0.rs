//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2135/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135<F: Float>(t24995: F, t34999: F, t5308: F, t28813: F, t6876: F, t19577: F, t22574: F, t33136: F, t19451: F, t6535: F, t28830: F, t31035: F) -> (F, F, F, F, F) {
    let t96805 = F::new(12.0) * t24995 * t34999 * t5308;
    let t96807 = F::new(2.0) * t6876 * t28813;
    let t96813 = F::new(6.0) * t22574 * t33136 * t19577;
    let t96815 = F::new(2.0) * t19451 * t6535;
    let t96818 = F::new(6.0) * t22574 * t31035 * t28830;
    (t96805, t96807, t96813, t96815, t96818)
}
