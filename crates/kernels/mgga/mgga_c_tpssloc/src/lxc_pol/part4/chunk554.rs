//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 554/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk554<F: Float>(t2887: F, t2764: F, t938: F, t942: F, t320: F, t941: F) -> (F, F, F, F) {
    let t2888 = F::new(1.0) / t2887;
    let t2892 = F::new(0.12361111111111111111e-1) * t2764;
    let t2900 = t938 * t942;
    let t2903 = t941 * t320;
    let t2904 = F::new(1.0) / t2903;
    (t2888, t2892, t2900, t2904)
}
