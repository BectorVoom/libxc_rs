//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1140/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1140<F: Float>(t1401: F, t1458: F, t2039: F, t3941: F, t5371: F, t577: F, t7230: F, t7801: F, t7945: F, t7956: F, t1714: F, t460: F) -> (F, F) {
    let t7961 = F::new(0.45e1) * t7945 * t577 + F::new(0.135e2) * t7230 * t1458 + F::new(0.135e2) * t5371 * t2039 + F::new(27.0) * t3941 * t7956 + F::new(0.135e2) * t1401 * t7801;
    let t8034 = t1714 * t460;
    (t7961, t8034)
}
