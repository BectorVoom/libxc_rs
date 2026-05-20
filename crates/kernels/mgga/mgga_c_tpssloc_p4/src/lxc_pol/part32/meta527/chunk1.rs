//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1862/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1862<F: Float>(t33: F, t7973: F, t2240: F, t12571: F, t7245: F, t1419: F, t55: F, t22510: F, t24498: F, t3961: F, t3966: F, t607: F, t7251: F) -> (F, F, F, F, F) {
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    let t27356 = t1419 * t55;
    let t27363 = F::new(20.0) / F::new(9.0) * t27356 * t607 + F::new(5.0) / F::new(18.0) * t24498 * t3961 - F::new(5.0) / F::new(6.0) * t7251 * t3966 - t22510;
    (t27331, t27332, t27341, t27356, t27363)
}
