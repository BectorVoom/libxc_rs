//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 566/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk566<F: Float>(t2932: F, t5811: F, t959: F, t2980: F, t5392: F, t2979: F, t4514: F, t4531: F, t2994: F, t977: F, t5398: F, t978: F) -> (F, F, F, F, F, F, F, F) {
    let t5812 = t5811 * t2932;
    let t5814 = F::new(0.17315859105681463759e2) * t959 * t5812;
    let t5817 = t2980 * t5392;
    let t5818 = t2979 * t5817;
    let t5821 = t4531 * t4514;
    let t5824 = t2994 * t5392;
    let t5825 = t977 * t5824;
    let t5828 = t978 * t5398;
    (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828)
}
