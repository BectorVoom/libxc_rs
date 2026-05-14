//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 887/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk887<F: Float>(t3053: F, t3117: F, t1043: F, t676: F, t248: F, t884: F, t1041: F, t3048: F, t10478: F, t3128: F, t10472: F, t10481: F, t3131: F, t1021: F, t1015: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10866 = t3117 * t3053;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    let t10871 = t1041 * t10870;
    let t10873 = t3048 * t3053;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10877 = t10481 * t3131;
    let t10879 = t248 * t1021 * t10877;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10884 = t10481 * t360;
    (t10866, t10868, t10870, t10871, t10873, t10875, t10876, t10877, t10879, t10882, t10883, t10884)
}
