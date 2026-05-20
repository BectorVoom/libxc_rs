//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2659/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659<F: Float>(t25: F, t1298: F, t15989: F, t15992: F, t16557: F, t19606: F, t20216: F, t20376: F, t2219: F, t3704: F, t39861: F, t5170: F, t606: F, t67059: F, t73975: F, t73978: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t74335 = piecewise3::<F>(t26, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39861 * t20376 * t606 + F::new(16.0) / F::new(9.0) * t19606 * t2219 + F::new(8.0) / F::new(9.0) * t15989 * t73975 - F::new(4.0) / F::new(3.0) * t15992 * t73978 - F::new(2.0) / F::new(3.0) * t5170 * t16557 - F::new(2.0) / F::new(9.0) * t3704 * t20216 * t606 + F::new(2.0) / F::new(3.0) * t1298 * t67059);
    t74335
}
