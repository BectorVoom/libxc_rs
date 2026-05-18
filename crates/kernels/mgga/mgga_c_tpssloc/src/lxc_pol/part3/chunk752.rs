//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 752/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk752<F: Float>(t210: F, t214: F, t4119: F, t2562: F, t2564: F, t2569: F, t2579: F, t2590: F, t4124: F, t4127: F, t4130: F, t4135: F, t787: F) -> (F, F) {
    let t4138 = t210 * t214 * t4119;
    let t4142 = t2562 + F::new(0.38888888888888888888e-2) * t2564 + t2569 + F::new(0.38888888888888888887e-2) * t4124 + F::new(0.49999999999999999998e-2) * t4127 * t4130 + F::new(0.8333333333333333333e-3) * t4135 - F::new(0.16666666666666666666e-2) * t787 * t4138 + F::new(0.83333333333333333332e-3) * t2579 - t2590;
    (t4138, t4142)
}
