//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 299/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk299<F: Float>(t912: F, t913: F, t893: F, t880: F, t886: F, t307: F) -> (F, F, F, F, F, F) {
    let t914 = t912 * t913;
    let t916 = F::new(1.0) * t893 * t914;
    let t917 = F::new(0.17123333333333333333e-1) * t880;
    let t919 = -t917 - F::new(0.17123333333333333333e-1) * t886;
    let t922 = t307 * t307;
    let t923 = F::new(1.0) / t922;
    (t914, t916, t917, t919, t922, t923)
}
