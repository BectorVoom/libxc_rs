//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 706/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk706<F: Float>(t1155: F, t3403: F, t4882: F, t1164: F, t1171: F, t1706: F, t1420: F, t972: F) -> (F, F, F, F, F) {
    let t4883 = t3403 * t1155;
    let t4884 = t4882 * t4883;
    let t4886 = F::new(0.17315859105681463759e2) * t1164 * t4884;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    (t4883, t4884, t4886, t4887, t4889)
}
