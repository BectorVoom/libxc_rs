//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1182/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1182<F: Float>(t31285: F, t3941: F, t1873: F, t649: F, t6534: F, t89: F, t645: F, t8513: F, t8514: F, t1862: F, t31: F, t607: F) -> (F, F, F, F, F, F) {
    let t31286 = t3941 * t31285;
    let t31287 = F::new(27.0) * t31286;
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31677 = t8513 * t8514 * t645;
    let t31682 = t1862 * t31;
    let t31683 = t31682 * t607;
    (t31287, t31537, t31540, t31677, t31682, t31683)
}
