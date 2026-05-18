//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1032/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1032<F: Float>(t11144: F, t285: F, t3907: F, t8833: F, t912: F, t2593: F, t3882: F, t905: F, t1448: F, t8749: F, t2595: F, t8752: F) -> (F, F, F, F, F) {
    let t11146 = F::new(0.621814e-1) * t11144 * t285;
    let t11147 = t3907 * t8833;
    let t11149 = F::new(0.17315859105681463759e2) * t912 * t11147;
    let t11152 = t2593 * t3882;
    let t11153 = t11152 * t905;
    let t11155 = F::new(0.23392894490538584828e1) * t912 * t11153;
    let t11156 = t8749 * t1448;
    let t11157 = t8752 * t2595;
    (t11146, t11149, t11155, t11156, t11157)
}
