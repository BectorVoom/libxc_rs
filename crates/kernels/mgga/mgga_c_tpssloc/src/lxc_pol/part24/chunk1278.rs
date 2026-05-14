//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1278/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1278<F: Float>(t2303: F, t645: F, t72: F, t39049: F, t6489: F, t2240: F, t2251: F, t2261: F, t43: F, t2267: F, t614: F, t38: F, t9287: F, t835: F, t10913: F, t1860: F, t1864: F, t2244: F, t22490: F, t2250: F, t22502: F, t22505: F, t22512: F, t22513: F, t22516: F, t22534: F, t22551: F, t44: F, t607: F, t6486: F, t6490: F, t6492: F, t6495: F, t6500: F, t6506: F, t6509: F, t6510: F, t67: F, t9258: F, t9276: F, t9288: F) -> (F,) {
    let t83771 = t72 * t2303 * t645;
    let t83775 = t39049 * t6489;
    let t83778 = t2240 * t2251;
    let t83788 = t2261 * t43;
    let t83791 = t614 * t2267;
    let t83796 = t38 * t9287;
    let t83803 = 1232.0 / 27.0 * t835;
    let t83812 = 2.0 * t6495 * t22516 + 5.0 / 2.0 * t6490 * t83771 + t6495 * t22490 + 5.0 / 2.0 * t83775 * t6492 - 5.0 * t83778 * t22551 + t22534 * t6506 + t22534 * t6510 - t6486 * t22513 / 2.0 - t6486 * t22516 - t1860 * (-1232.0 / 27.0 * t9276 * t44 + 220.0 / 9.0 * t83788 * t607 - 20.0 / 9.0 * t83791 * t2244 - 20.0 / 3.0 * t22502 * t2250 - 5.0 / 108.0 * t83796 * t9288 + 5.0 / 6.0 * t22505 * t10913 + 5.0 / 6.0 * t6500 * t9258 + t83803) * t67 * t1864 / 6.0 - t1860 * t22512 * t6509 / 2.0;
    (t83812,)
}
