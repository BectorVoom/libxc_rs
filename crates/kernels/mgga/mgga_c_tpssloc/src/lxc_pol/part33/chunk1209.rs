//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1209/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1209<F: Float>(t40610: F, t2751: F, t10108: F, t257: F, t68: F, t10163: F, t386: F, t3215: F, t1406: F, t9238: F, t111: F, t6470: F) -> (F, F, F, F, F, F, F) {
    let t40611 = F::new(1.0) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::new(1.0) / t40771;
    let t40889 = F::new(1.0) / t10108 / t257;
    let t40890 = t68 * t40889;
    let t43603 = F::new(1.0) / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43636 = t3215 * t3215;
    let t43637 = F::new(1.0) / t43636;
    let t45844 = t1406 * t9238;
    let t55388 = t6470 * t111;
    (t40611, t40772, t40890, t43604, t43637, t45844, t55388)
}
