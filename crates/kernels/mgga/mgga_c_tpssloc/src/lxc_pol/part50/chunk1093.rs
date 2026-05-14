//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1093/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1093<F: Float>(t1081: F, t7540: F, t118953: F, t25927: F, t1649: F, t6665: F, t25374: F, t89953: F, t16596: F, t25353: F, t28: F, t118454: F, t23788: F, t113135: F, t118376: F, t118381: F, t118436: F, t118465: F, t118949: F, t1877: F, t22959: F, t23290: F, t2522: F, t25372: F, t25892: F, t25901: F, t25905: F, t25928: F, t25934: F, t25938: F, t25945: F, t30753: F, t30757: F, t30770: F, t32886: F, t33065: F, t6670: F, t6841: F, t8366: F) -> (F,) {
    let t119737 = t1081 * t7540;
    let t119743 = t25927 * t118953;
    let t119746 = t1649 * t6665;
    let t119755 = t89953 * t25374;
    let t119763 = t25927 * t16596;
    let t119766 = t28 * t25353;
    let t119780 = t23788 * t118454;
    let t119783 = -t1877 * t30757 * t25934 / 2.0 - t1877 * t6670 * t119737 + t1877 * t30770 * t25945 + t118436 * t25928 + 2.0 * t25372 * t119743 - t1877 * t6670 * t119746 + t1877 * t30753 * t1649 / 2.0 + 3.0 / 2.0 * t2522 * t32886 * t6841 - 3.0 * t118376 * t119755 + 3.0 / 2.0 * t2522 * t8366 * t25901 + 3.0 * t118381 * t25892 + 3.0 * t113135 * t119763 - t118465 - t1877 * t6670 * t119766 - t1877 * t23290 * t33065 + 3.0 / 2.0 * t2522 * t8366 * t25905 + t1877 * t118949 * t28 / 2.0 + 3.0 / 2.0 * t2522 * t8366 * t25938 - 3.0 * t22959 * t119780;
    (t119783,)
}
