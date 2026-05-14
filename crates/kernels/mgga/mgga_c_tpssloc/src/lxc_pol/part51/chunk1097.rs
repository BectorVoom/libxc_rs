//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1097/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1097<F: Float>(t7458: F, t8533: F, t1873: F, t7890: F, t652: F, t1458: F, t2035: F) -> (F, F, F, F) {
    let t33230 = 2.0 * t7458 * t8533;
    let t33231 = t7890 * t1873;
    let t33233 = 2.0 * t652 * t33231;
    let t33234 = t2035 * t1458;
    (t33230, t33231, t33233, t33234)
}
