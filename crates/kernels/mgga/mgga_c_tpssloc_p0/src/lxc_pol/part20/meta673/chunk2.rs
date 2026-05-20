//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2540/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540<F: Float>(t11419: F, t1675: F, t11424: F, t15054: F, t15057: F, t44162: F, t11185: F, t15064: F, t15068: F, t43964: F, t3264: F, t3307: F, t4782: F) -> (F, F, F, F, F, F) {
    let t51427 = t1675 * t11419;
    let t51437 = F::new(6.0) * t11424 * t15054;
    let t51439 = F::cast_from(0.28947563097646563121e3_f64) * t44162 * t15057;
    let t51441 = F::cast_from(0.48245938496077605201e2_f64) * t11185 * t15064;
    let t51443 = F::cast_from(0.1551780387578202009e4_f64) * t43964 * t15068;
    let t51446 = F::new(6.0) * t3264 * t4782 * t3307;
    (t51427, t51437, t51439, t51441, t51443, t51446)
}
