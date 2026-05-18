//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 153/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk153<F: Float>(t407: F, t410: F, t413: F, t417: F) -> (F, F, F) {
    let t445 = F::new(0.51785e1) * t410 + F::new(0.905775e0) * t407 + F::new(0.1100325e0) * t413 + F::new(0.1241775e0) * t417;
    let t448 = F::new(1.0) + F::new(0.29608749977793437516e2) / t445;
    let t449 = f64::ln(t448);
    (t445, t448, t449)
}
