//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 943/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk943<F: Float>(t1339: F, t26322: F, t6936: F, t22856: F, t22859: F, t22860: F, t22864: F, t22868: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F, t22766: F, t22780: F, t22798: F, t22805: F, t22820: F, t22826: F, t26231: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26251: F, t26280: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F) -> (F, F) {
    let t26323 = t1339 * t26322;
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / 384.0 + t26310 / 768.0 - t26312 / 1536.0 + t26314 / 384.0 + 0.33643963411783659045e-4 * t22856 + t22859 - 7.0 / 2304.0 * t22860 + t22864 + t22868 + 0.40372756094140390854e-3 * t26320 - 0.20186378047070195427e-3 * t26324;
    let t26328 = 7.0 / 2304.0 * t26231 - t26234 / 1536.0 - t26236 / 1536.0 - t26238 / 1536.0 + 5.0 / 384.0 * t26240 + 7.0 / 2304.0 * t22766 + 0.33643963411783659045e-4 * t26246 + t26249 / 1536.0 - 7.0 / 2304.0 * t26251 + 0.14130464632949136799e-2 * t22780 + t26280 + 7.0 / 144.0 * t22798 + 0.84782787797694820794e-2 * t22805 - t22820 + t22826 + t26286 / 16.0 + 0.84782787797694820792e-2 * t26290 - 0.20186378047070195427e-3 * t26293 + 0.14130464632949136799e-2 * t26295 + 0.12111826828242117256e-2 * t26299 + 0.12111826828242117256e-2 * t26303 + t26326;
    (t26324, t26328)
}
