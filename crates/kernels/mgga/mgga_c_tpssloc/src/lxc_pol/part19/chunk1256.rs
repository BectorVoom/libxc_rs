//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1256/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1256<F: Float>(t11013: F, t225: F, t10163: F, t386: F, t68: F, t3175: F, t11008: F, t10160: F, t10165: F, t10167: F, t10170: F, t1052: F, t1055: F, t1058: F, t1060: F, t1061: F, t1065: F, t1066: F, t11010: F, t11024: F, t11027: F, t11028: F, t11034: F, t11046: F, t11048: F, t11051: F, t11054: F, t11061: F, t11067: F, t11077: F, t11078: F, t11084: F, t11085: F, t14630: F, t3026: F, t3076: F, t3120: F, t3166: F, t3169: F, t3174: F, t3176: F, t3180: F, t3186: F, t3188: F, t3192: F, t3193: F, t3196: F, t3197: F, t3200: F, t3202: F, t3204: F, t3206: F, t3207: F, t381: F, t384: F, t388: F, t42715: F, t43082: F, t43083: F, t43470: F, t43473: F, t43483: F, t43504: F, t43512: F, t43515: F, t43516: F, t43525: F, t43536: F, t43542: F, t43584: F, t4684: F) -> (F,) {
    let t43599 = t11013 * t225;
    let t43603 = 1.0 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43605 = t3175 * t3175;
    let t43619 = t11008 * t225;
    let t43622 = -6.0 * t10170 * t3207 - t1052 * t1055 * (6.0 * t1058 * t3166 * t3120 * t1060 - 4.0 * t3200 * t11027 * t4684 + 24.0 * t3186 * t3192 * t11054 - 12.0 * t3200 * t11077 * t4684 + 12.0 * t11051 * t3193 + 24.0 * t43473 * t11061 - 24.0 * t43470 * t11067 + 12.0 * t3180 * t11078 + 6.0 * t3076 * t3204 + t43083 * t384 + t43512 + t1058 * t381 * t42715 * t1060 + 4.0 * t11046 * t43483 * t11048 + 6.0 * t11046 * t3196 * t14630 + 6.0 * t3186 * t43525 * t3188 + 14.0 * t43515 * t43504 * t43516 + 4.0 * t43542 * t1061 + 24.0 * t11034 * t11024 + 4.0 * t3180 * t11028 + 6.0 * t11051 * t3197 - 6.0 * t43536 * t3202 + t43584) + 12.0 * t11010 * t3176 - 24.0 * t3026 * t10167 - 24.0 * t3169 * t10167 - 12.0 * t10160 * t3207 + t43082 * t381 * t388 - 12.0 * t43599 * t1066 + 24.0 * t1052 * t43604 * t43605 - 4.0 * t3026 * t11085 - 36.0 * t1052 * t10165 * t3175 * t3206 + 8.0 * t1052 * t3174 * t1065 * t11084 - 4.0 * t43619 * t1066;
    (t43622,)
}
