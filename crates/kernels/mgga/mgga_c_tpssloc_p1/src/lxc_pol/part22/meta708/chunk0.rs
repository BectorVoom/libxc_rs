//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2302/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302<F: Float>(t52: F, t12874: F, t12877: F, t16558: F, t16563: F, t17635: F, t20217: F, t20234: F, t2440: F, t3966: F, t40647: F, t4087: F, t5398: F, t607: F, t67060: F, t76: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t67082 = piecewise3::<F>(t150, F::new(0.0), F::new(40.0) / F::new(81.0) * t40647 * t20234 * t607 + F::new(8.0) / F::new(9.0) * t16563 * t3966 + F::new(8.0) / F::new(9.0) * t12874 * t17635 + F::new(4.0) / F::new(3.0) * t12877 * t5398 + F::new(4.0) / F::new(3.0) * t4087 * t16558 + F::new(4.0) / F::new(9.0) * t2440 * t20217 * t607 - F::new(4.0) / F::new(3.0) * t76 * t67060);
    t67082
}
