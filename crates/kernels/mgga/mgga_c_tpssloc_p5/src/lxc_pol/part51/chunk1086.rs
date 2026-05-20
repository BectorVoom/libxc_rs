//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1086/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1086<F: Float>(t26446: F, t26447: F, t26331: F, t26403: F, t5250: F, t5287: F, t6987: F, t1338: F, t7722: F, t1352: F, t16036: F, t550: F) -> (F, F, F, F, F) {
    let t26448 = t26446 * t26447;
    let t26449 = t26331 * t26448;
    let t26453 = t26403 * t5250;
    let t26456 = t6987 * t5287;
    let t26458 = t1338 * t7722;
    let t26459 = t26458 * t1352;
    let t26461 = t16036 * t550;
    (t26449, t26453, t26456, t26459, t26461)
}
