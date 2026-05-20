//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1291/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1291<F: Float>(t28: F, t1081: F, t5142: F, t5145: F, t584: F, t157: F, t5141: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t5149 = piecewise3::<F>(t29, F::new(0.0), F::new(4.0) / F::new(9.0) * t5142 * t1081 - F::new(8.0) / F::new(3.0) * t5145 * t584);
    let t5151 = (t5141 + t5149) * t157;
    t5151
}
