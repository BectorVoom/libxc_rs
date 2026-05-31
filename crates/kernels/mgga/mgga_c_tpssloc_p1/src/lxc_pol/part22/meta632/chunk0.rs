//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2167/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2167<F: Float>(t15892: F, t2535: F, t2528: F, t40225: F, t15921: F, t588: F, t15971: F, t12364: F, t5234: F, t1354: F, t12365: F, t5289: F) -> (F, F, F, F, F, F, F, F) {
    let t54469 = t15892 * t2535;
    let t54470 = F::cast_from(0.17544670867903938621e1_f64) * t54469;
    let t54471 = t15892 * t2528;
    let t54472 = F::cast_from(0.51947577317044391276e2_f64) * t54471;
    let t54473 = F::cast_from(36.0_f64) * t40225;
    let t54475 = F::cast_from(24.0_f64) * t588 * t15921;
    let t54477 = t588 * t15971;
    let t54478 = F::cast_from(12.0_f64) * t54477;
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54534 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54533;
    let t54555 = t12365 * t5289;
    (t54470, t54472, t54473, t54475, t54478, t54532, t54534, t54555)
}
