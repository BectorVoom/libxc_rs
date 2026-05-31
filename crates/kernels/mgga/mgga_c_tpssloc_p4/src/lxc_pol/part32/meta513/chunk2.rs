//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1842/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1842<F: Float>(t22766: F, t22780: F, t22798: F, t22805: F, t22820: F, t22826: F, t26231: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26251: F, t26280: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t26326: F) -> F {
    let t26328 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t26231 - t26234 / F::cast_from(1536.0_f64) - t26236 / F::cast_from(1536.0_f64) - t26238 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t26240 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t22766 + F::cast_from(0.33643963411783659045e-4_f64) * t26246 + t26249 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t26251 + F::cast_from(0.14130464632949136799e-2_f64) * t22780 + t26280 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t22798 + F::cast_from(0.84782787797694820794e-2_f64) * t22805 - t22820 + t22826 + t26286 / F::cast_from(16.0_f64) + F::cast_from(0.84782787797694820792e-2_f64) * t26290 - F::cast_from(0.20186378047070195427e-3_f64) * t26293 + F::cast_from(0.14130464632949136799e-2_f64) * t26295 + F::cast_from(0.12111826828242117256e-2_f64) * t26299 + F::cast_from(0.12111826828242117256e-2_f64) * t26303 + t26326;
    t26328
}
