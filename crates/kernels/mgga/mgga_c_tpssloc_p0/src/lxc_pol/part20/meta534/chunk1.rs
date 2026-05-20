//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2072/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2072<F: Float>(t12231: F, t3726: F, t12199: F, t12208: F, t118: F, t12012: F, t3739: F, t794: F, t12217: F, t40021: F, t3774: F, t3862: F) -> (F, F, F, F, F) {
    let t40423 = t3726 * t12231;
    let t40425 = t12199 * t12208;
    let t40429 = t3739 * t118 * t794 * t12012;
    let t40431 = t40021 * t12217;
    let t40443 = t3774 * t3862;
    (t40423, t40425, t40429, t40431, t40443)
}
