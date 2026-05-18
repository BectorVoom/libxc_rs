//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1003/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1003<F: Float>(t22766: F, t22780: F, t22798: F, t22805: F, t22820: F, t22826: F, t26231: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26251: F, t26280: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t26326: F) -> F {
    let t26328 = F::new(7.0) / F::new(2304.0) * t26231 - t26234 / F::new(1536.0) - t26236 / F::new(1536.0) - t26238 / F::new(1536.0) + F::new(5.0) / F::new(384.0) * t26240 + F::new(7.0) / F::new(2304.0) * t22766 + F::new(0.33643963411783659045e-4) * t26246 + t26249 / F::new(1536.0) - F::new(7.0) / F::new(2304.0) * t26251 + F::new(0.14130464632949136799e-2) * t22780 + t26280 + F::new(7.0) / F::new(144.0) * t22798 + F::new(0.84782787797694820794e-2) * t22805 - t22820 + t22826 + t26286 / F::new(16.0) + F::new(0.84782787797694820792e-2) * t26290 - F::new(0.20186378047070195427e-3) * t26293 + F::new(0.14130464632949136799e-2) * t26295 + F::new(0.12111826828242117256e-2) * t26299 + F::new(0.12111826828242117256e-2) * t26303 + t26326;
    t26328
}
