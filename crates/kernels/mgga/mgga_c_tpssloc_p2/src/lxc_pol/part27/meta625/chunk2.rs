//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2109/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2109<F: Float>(t45560: F, t7769: F, t16521: F, t6534: F, t111: F, t7758: F, t1873: F, t55405: F, t16524: F, t23893: F, t12524: F, t26550: F) -> (F, F, F, F, F, F) {
    let t86642 = F::new(27.0) * t45560 * t7769;
    let t86646 = F::new(27.0) * t16521 * t6534;
    let t86647 = t7758 * t111;
    let t86651 = F::new(27.0) * t55405 * t1873;
    let t86653 = F::new(54.0) * t16524 * t23893;
    let t86655 = F::new(54.0) * t12524 * t26550;
    (t86642, t86646, t86647, t86651, t86653, t86655)
}
