//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 512/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk512<F: Float>(t343: F, t883: F, t2775: F, t344: F, t2822: F, t1008: F, t191: F) -> (F, F, F, F) {
    let t2989 = t343 * t883;
    let t2994 = t344 * t2775;
    let t3003 = F::new(5.0) / F::new(18.0) * t2822;
    let t3030 = F::new(1.0) / t1008 / t191;
    (t2989, t2994, t3003, t3030)
}
