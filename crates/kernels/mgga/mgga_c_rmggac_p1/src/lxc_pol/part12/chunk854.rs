//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 854/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk854<F: Float>(t1182: F, t551: F, t1970: F, t209: F, t236: F, t3352: F, t7244: F, t9159: F, t1971: F, t3351: F, t5156: F, t7190: F) -> (F, F, F, F) {
    let t38928 = t551 * t1182;
    let t38932 = t1970 * t3352 * t236 * t38928 * t209;
    let t38934 = t7244 * t9159;
    let t38938 = t3351 * t1971 * t7190 * t5156;
    (t38928, t38932, t38934, t38938)
}
