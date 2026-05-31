//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2668/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668<F: Float>(t54412: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F, t39496: F, t54401: F, t54403: F, t54409: F, t74056: F, t74057: F, t74073: F, t74075: F, t74078: F, t74086: F) -> (F, F) {
    let t74470 = F::cast_from(36.0_f64) * t54412;
    let t74471 = -t74056 + t39463 - t39468 + t74057 + t54401 - t39472 - t39476 - t54403 + t74073 - t74075 - t74078 + t54409 + t74086 + t39483 - t74470 - t39490 - t39496;
    (t74470, t74471)
}
