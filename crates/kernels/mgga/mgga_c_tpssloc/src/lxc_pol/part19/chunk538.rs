//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 538/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk538<F: Float>(t2793: F, t913: F, t2792: F, t273: F, t276: F, t896: F) -> (F, F, F, F) {
    let t2794 = t2793 * t913;
    let t2796 = 2.0 * t2792 * t2794;
    let t2798 = 1.0 / t276 / t273;
    let t2799 = t896 * t896;
    (t2794, t2796, t2798, t2799)
}
