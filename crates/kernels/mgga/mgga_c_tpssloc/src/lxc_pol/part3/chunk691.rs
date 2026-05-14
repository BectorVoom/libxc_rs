//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 691/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk691<F: Float>(t1369: F, t3866: F, t1995: F, t241: F, t67: F, t3734: F, t820: F, t1367: F, t3719: F, t1315: F, t1341: F, t1354: F, t1363: F, t3733: F, t3762: F, t3763: F, t3766: F, t3770: F, t3774: F, t3778: F, t3781: F, t3783: F, t3790: F, t3795: F, t3800: F, t3803: F, t3809: F, t3853: F, t3858: F, t3864: F, t559: F) -> (F, F, F, F, F) {
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3872 = t3870 * t820 * t3734;
    let t3876 = t1367 * t820 * t3719;
    let t3879 = t3762 + 7.0 / 72.0 * t3763 + t3733 * t3766 / 16.0 - t1315 * t3770 / 48.0 + t3774 * t559 / 3072.0 - t3778 * t1354 / 1536.0 - 7.0 / 2304.0 * t3781 - t3783 * t1369 / 384.0 + t3790 * t3795 / 1536.0 + 7.0 / 2304.0 * t3800 + t3803 * t3809 / 384.0 - t1341 * t3853 / 3072.0 - t1341 * t3858 / 3072.0 + t3864 + 7.0 / 576.0 * t3867 + 5.0 / 768.0 * t1363 * t3872 - t1363 * t3876 / 768.0;
    (t3867, t3870, t3872, t3876, t3879)
}
