//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1030/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1030<F: Float>(t1525: F, t1971: F, t511: F, t558: F, t7230: F, t1737: F, t495: F, t880: F, t10018: F, t7244: F, t7255: F, t9985: F) -> (F, F, F, F) {
    let t47505 = t7230 * t1971 * t511 * t558 * t1525;
    let t47510 = t7230 * t1971 * t880 * t1737 * t495;
    let t47512 = t7244 * t10018;
    let t47516 = t7255 * t9985;
    (t47505, t47510, t47512, t47516)
}
