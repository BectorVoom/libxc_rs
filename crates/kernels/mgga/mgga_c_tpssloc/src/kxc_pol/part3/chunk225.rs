//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 225/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk225<F: Float>(t688: F, t690: F, t694: F, t699: F, t141: F) -> (F, F) {
    let t701 = -F::new(0.632975e0) * t688 - F::new(0.29896666666666666667e0) * t690 - F::new(0.1023875e0) * t694 - F::new(0.82156666666666666667e-1) * t699;
    let t702 = F::new(1.0) / t141;
    (t701, t702)
}
