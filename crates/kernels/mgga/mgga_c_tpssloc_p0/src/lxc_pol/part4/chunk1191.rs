//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1191/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1191<F: Float>(t28: F, t12000: F, t6312: F, t3711: F, t5966: F, t1081: F, t1302: F, t18196: F, t2219: F, t5178: F, t19617: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t19618 = t12000 * t6312;
    let t19623 = t3711 * t5966;
    let t19629 = piecewise3::<F>(t29, F::new(0.0), F::new(8.0) / F::new(27.0) * t19618 * t1081 + F::new(8.0) / F::new(9.0) * t5178 * t2219 - F::new(2.0) / F::new(9.0) * t19623 * t1081 + F::new(2.0) / F::new(3.0) * t1302 * t18196);
    let t19631 = t19617 / F::new(2.0) + t19629 / F::new(2.0);
    t19631
}
