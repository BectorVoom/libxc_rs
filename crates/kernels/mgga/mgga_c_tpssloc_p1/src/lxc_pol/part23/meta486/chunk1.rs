//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1491/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491<F: Float>(t28: F, t6312: F, t5966: F, t19559: F, t20390: F, t3672: F, t39436: F, t5142: F, t517: F, t77953: F, t157: F, t79872: F, t182: F, zeta_threshold: F) -> (F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t79873 = t6312 * t6312;
    let t79878 = t5966 * t5966;
    let t79886 = piecewise3::<F>(t29, F::new(0.0), F::new(40.0) / F::new(81.0) * t39436 * t79873 - F::new(16.0) / F::new(9.0) * t19559 * t5966 + F::new(4.0) / F::new(3.0) * t3672 * t79878 + F::new(16.0) / F::new(9.0) * t5142 * t20390 + F::new(4.0) / F::new(3.0) * t517 * t77953);
    let t79888 = (t79872 + t79886) * t157;
    let t79890 = F::cast_from(0.19751673498613801407e-1_f64) * t79888 * t182;
    (t79873, t79878, t79888, t79890)
}
