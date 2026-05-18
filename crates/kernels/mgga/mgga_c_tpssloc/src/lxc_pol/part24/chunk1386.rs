//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1386/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1386<F: Float>(t23413: F, t344: F, t6740: F, t1016: F, t3034: F, t1930: F, t6741: F, t10469: F, t10474: F, t363: F, t10481: F, t3032: F) -> (F, F, F, F) {
    let t82981 = t6740 * t23413 * t344;
    let t82985 = F::new(1.0) / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t82987 = t82986 * t6741;
    let t82989 = t10469 * t10474 * t363;
    let t82990 = t10481 * t3032;
    (t82981, t82987, t82989, t82990)
}
