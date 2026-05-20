//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 854/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk854<F: Float>(t25: F, t28: F, t1408: F, t3664: F, t514: F, t5397: F, t1649: F, t3672: F, t517: F, t5966: F, t157: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t6305 = t1408 * t1408;
    let t6311 = piecewise3::<F>(t26, F::new(0.0), F::new(4.0) / F::new(9.0) * t3664 * t6305 + F::new(4.0) / F::new(3.0) * t514 * t5397);
    let t6312 = t1649 * t1649;
    let t6318 = piecewise3::<F>(t29, F::new(0.0), F::new(4.0) / F::new(9.0) * t3672 * t6312 + F::new(4.0) / F::new(3.0) * t517 * t5966);
    let t6320 = (t6311 + t6318) * t157;
    (t6305, t6312, t6320)
}
