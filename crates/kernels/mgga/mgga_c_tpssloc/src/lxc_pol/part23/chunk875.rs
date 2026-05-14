//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 875/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk875<F: Float>(t19681: F, t763: F, t1819: F, t68: F, t1995: F, t6330: F, t1824: F, t1834: F, t562: F, t6387: F, t118: F, t794: F, t12202: F, t6347: F, t3739: F, t12211: F, t6353: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19682 = t19681 * t763;
    let t19708 = t1819 * t68;
    let t19715 = t1995 * t6330;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19775 = t118 * t794 * t6347;
    let t19776 = t3739 * t19775;
    let t19779 = t12211 * t6353;
    (t19682, t19708, t19715, t19739, t19743, t19767, t19768, t19775, t19776, t19779)
}
