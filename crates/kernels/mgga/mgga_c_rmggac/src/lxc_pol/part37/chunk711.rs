//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 711/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk711<F: Float>(t21708: F, t74703: F, t9189: F, t21709: F, t9193: F, t15238: F, t9128: F, t1550: F, t2060: F, t41059: F, t14362: F, t2411: F, t3144: F, t577: F, t669: F, t7933: F, t7934: F) -> (F, F, F, F, F, F) {
    let t74705 = t21708 * t74703 * t9189;
    let t74708 = t21708 * t21709 * t9193;
    let t74713 = 0.5987120850931904282e-1 * t9128 * t15238;
    let t74716 = 0.5987120850931904282e-1 * t1550 * t2060 * t41059;
    let t74718 = t2411 * t14362 * t3144;
    let t74722 = t7933 * t7934 * t577 * t669;
    (t74705, t74708, t74713, t74716, t74718, t74722)
}
