//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 654/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk654<F: Float>(t1484: F, t2523: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2522: F, t2530: F, t2537: F, t2538: F, t2665: F, t4209: F, t4213: F, t4214: F, t4215: F, t4216: F) -> F {
    let t4320 = t2523 * t1484;
    let t4323 = F::cast_from(3.0_f64) * t2522 * t4320 + t2408 + t2417 - t2423 - t2426 - t2486 + t2518 - t2530 - t2537 + t2538 + t2665 + t4209 - t4213 + t4214 - t4215 - t4216;
    t4323
}
