//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 829/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk829<F: Float>(t40: F, t52: F, t2250: F, t73: F, t9258: F, t9288: F, t9427: F, t9430: F, t197: F, t2440: F, t607: F, t76: F, t145: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t9436 = piecewise3::<f64>(t146, F::new(0.0), -F::new(8.0) / F::new(27.0) * t9427 * t9288 + F::new(4.0) / F::new(3.0) * t9430 * t2250 + F::new(4.0) / F::new(3.0) * t73 * t9258);
    let t9438 = F::new(1.0) / t197 / t52;
    let t9441 = t2440 * t607;
    let t9447 = piecewise3::<f64>(t150, F::new(0.0), F::new(8.0) / F::new(27.0) * t9438 * t9288 + F::new(4.0) / F::new(3.0) * t9441 * t2250 - F::new(4.0) / F::new(3.0) * t76 * t9258);
    let t9448 = t9436 + t9447;
    let t9449 = t145 * t9448;
    (t9448, t9449)
}
