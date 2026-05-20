//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2674/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2674<F: Float>(t54467: F, t15892: F, t2535: F, t2528: F, t40225: F, t15921: F, t588: F, t40231: F, t15971: F, t40224: F, t40230: F, t54455: F, t54457: F, t54459: F, t54461: F, t54463: F, t54464: F, t54465: F, t54466: F) -> (F, F, F, F, F, F, F, F) {
    let t54468 = F::cast_from(0.10254018858216406658e4_f64) * t54467;
    let t54469 = t15892 * t2535;
    let t54470 = F::cast_from(0.17544670867903938621e1_f64) * t54469;
    let t54471 = t15892 * t2528;
    let t54472 = F::cast_from(0.51947577317044391276e2_f64) * t54471;
    let t54473 = F::new(36.0) * t40225;
    let t54475 = F::new(24.0) * t588 * t15921;
    let t54476 = F::new(36.0) * t40231;
    let t54477 = t588 * t15971;
    let t54478 = F::new(12.0) * t54477;
    let t54479 = t54455 + t54457 - t54459 + t54461 - t54463 - t54464 + t54465 - t54466 - t54468 - t54470 - t54472 + t40224 - t54473 + t54475 - t40230 - t54476 + t54478;
    (t54468, t54470, t54472, t54473, t54475, t54476, t54478, t54479)
}
