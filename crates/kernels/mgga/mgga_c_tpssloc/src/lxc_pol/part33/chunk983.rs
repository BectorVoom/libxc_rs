//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 983/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk983<F: Float>(t10784: F, t10785: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F) -> F {
    let t21158 = F::new(0.20839e0) * t21120 - F::new(0.103295e1) * t21124 + F::new(0.309885e1) * t21128 - F::new(0.46308888888888888889e-1) * t21132 - F::new(0.104195e0) * t21136 - F::new(0.62517e0) * t21140 - F::new(0.52945875e1) * t21142 + F::new(0.94674375e0) * t21144 - t10784 - t10785 - F::new(0.57386111111111111112e0) * t21147 + F::new(0.20659e1) * t21150 - F::new(0.309885e1) * t21153 - F::new(0.516475e0) * t21156;
    t21158
}
