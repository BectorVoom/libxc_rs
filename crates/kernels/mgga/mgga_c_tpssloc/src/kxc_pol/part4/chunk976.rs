//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 976/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk976<F: Float>(t17766: F, t17798: F, t17852: F, t17873: F, t225: F, t68: F, t369: F, t10457: F, t248: F, t5677: F, t1041: F, t1044: F, t17187: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F, t973: F) -> (F, F, F) {
    let t17875 = t17766 + t17798 + t17852 + t17873;
    let t17876 = t17875 * t225;
    let t17877 = t17876 * t68;
    let t17878 = t17877 * t369;
    let t17884 = t248 * t10457 * t5677;
    let t17885 = t1041 * t17884;
    let t17890 = t248 * t1044 * t17187;
    let t17900 = t14084 + t14508 * t4596 / 768.0 - t14511 * t4600 / 1536.0 + t3130 * t17734 / 768.0 + t973 * t17738 / 288.0 + t17878 * t378 / 3072.0 - 5.0 / 2592.0 * t3048 * t5861 + 5.0 / 20736.0 * t17885 + t3117 * t5857 / 4608.0 + t1041 * t17890 / 4608.0 + 5.0 / 13824.0 * t3117 * t5861 + t14085 * t1622 / 2304.0 + t4644 * t4636 / 2304.0 - t14117 / 6912.0;
    (t17875, t17876, t17900)
}
