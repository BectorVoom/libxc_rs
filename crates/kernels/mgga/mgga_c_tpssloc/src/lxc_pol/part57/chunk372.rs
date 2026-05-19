//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 372/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk372<F: Float>(t212: F, t2587: F, t2586: F, t154: F, t2559: F, t222: F, t233: F, t813: F) -> (F, F, F, F) {
    let t2588 = t2587 * t212;
    let t2590 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t2588;
    let t2600 = t2559 * t154;
    let t2602 = F::new(35.0) / F::new(432.0) * t2600 * t222;
    let t2627 = F::new(1.0) / t813 / t233;
    (t2590, t2600, t2602, t2627)
}
