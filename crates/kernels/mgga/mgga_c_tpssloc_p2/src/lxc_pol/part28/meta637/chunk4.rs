//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2036/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2036<F: Float>(t92161: F, t92210: F, t93275: F, t93930: F, t93978: F, t94022: F, t94061: F, t94103: F, t1404: F, t7945: F, t2105: F, t5363: F) -> (F, F, F) {
    let t94106 = t92161 + t92210 + t93275 + t93930 + t93978 + t94022 + t94061 + t94103;
    let t94113 = F::new(2.0) * t7945 * t1404;
    let t94118 = F::new(2.0) * t5363 * t2105;
    (t94106, t94113, t94118)
}
