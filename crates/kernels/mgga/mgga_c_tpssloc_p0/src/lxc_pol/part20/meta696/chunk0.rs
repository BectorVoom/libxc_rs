//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2654/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654<F: Float>(t12283: F, t16271: F, t16383: F, t16370: F, t16060: F, t3798: F, t1354: F, t12345: F, t5310: F, t12339: F, t16150: F, t3866: F) -> (F, F, F, F, F, F, F) {
    let t54114 = t12283 * t16271;
    let t54116 = t12283 * t16383;
    let t54118 = t12283 * t16370;
    let t54124 = t16060 * t3798;
    let t54125 = t54124 * t1354;
    let t54131 = t12345 * t5310;
    let t54132 = F::cast_from(595.0_f64) / F::cast_from(1152.0_f64) * t54131;
    let t54133 = t12339 * t5310;
    let t54135 = t3866 * t16150;
    (t54114, t54116, t54118, t54125, t54132, t54133, t54135)
}
