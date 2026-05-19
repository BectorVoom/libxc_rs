//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1059/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1059<F: Float>(t74594: F, t74616: F, t68739: F, t74598: F, t74600: F, t74603: F, t74605: F, t74609: F, t74610: F, t77117: F, t77119: F, t77121: F, t77123: F, t77125: F, t77127: F, t77129: F, t77132: F) -> F {
    let t80136 = F::cast_from(0.15372131649401827112e-4_f64) * t74594;
    let t80138 = F::cast_from(0.49700494569958178262e-1_f64) * t74616;
    let t80139 = -t80136 + t77117 + t77119 - t74598 - t74600 - t74603 - t74605 - t74609 + t77121 - F::cast_from(0.31062809106223861414e-2_f64) * t74610 + t68739 + t77123 - t77125 - t77127 - t77129 - t80138 - t77132;
    t80139
}
