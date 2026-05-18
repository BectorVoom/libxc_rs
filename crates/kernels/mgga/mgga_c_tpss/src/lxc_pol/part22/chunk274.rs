//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 274/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk274<F: Float>(t865: F, t866: F, t846: F, t833: F, t839: F, t301: F) -> (F, F, F, F, F, F) {
    let t867 = t865 * t866;
    let t869 = F::new(1.0) * t846 * t867;
    let t870 = F::new(0.17123333333333333333e-1) * t833;
    let t872 = -t870 - F::new(0.17123333333333333333e-1) * t839;
    let t875 = t301 * t301;
    let t876 = F::new(1.0) / t875;
    (t867, t869, t870, t872, t875, t876)
}
