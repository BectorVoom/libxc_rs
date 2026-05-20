//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2121/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2121<F: Float>(t1404: F, t8110: F, t1851: F, t7426: F, t27907: F, t580: F, t2169: F, t5381: F, t1395: F, t8119: F, t1858: F, t7415: F) -> (F, F, F, F, F, F) {
    let t96283 = F::new(2.0) * t8110 * t1404;
    let t96285 = F::new(2.0) * t1851 * t7426;
    let t96289 = F::new(2.0) * t27907 * t580;
    let t96291 = F::new(2.0) * t2169 * t5381;
    let t96300 = F::new(2.0) * t1395 * t8119;
    let t96303 = F::new(2.0) * t7415 * t1858;
    (t96283, t96285, t96289, t96291, t96300, t96303)
}
