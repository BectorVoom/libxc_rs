//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 582/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk582<F: Float>(t40: F, t52: F, t4072: F, t510: F, t1774: F, t671: F, t1409: F, t2433: F, t3966: F, t607: F, t73: F, t2440: F, t76: F, t157: F, zeta_threshold: F) -> (F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t4073 = t510 * t4072;
    let t4077 = t1774 * t671;
    let t4080 = t2433 * t1409;
    let t4086 = piecewise3::<f64>(t146, F::new(0.0), F::new(4.0) / F::new(9.0) * t4080 * t607 + F::new(4.0) / F::new(3.0) * t73 * t3966);
    let t4087 = t2440 * t1409;
    let t4093 = piecewise3::<f64>(t150, F::new(0.0), F::new(4.0) / F::new(9.0) * t4087 * t607 - F::new(4.0) / F::new(3.0) * t76 * t3966);
    let t4094 = t4086 + t4093;
    let t4095 = t4094 * t157;
    (t4073, t4077, t4094, t4095)
}
