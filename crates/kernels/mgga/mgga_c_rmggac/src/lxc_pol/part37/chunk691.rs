//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 691/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk691<F: Float>(t16503: F, t35039: F, t665: F, t9169: F, t15405: F, t34761: F, t1502: F, t34976: F, t2010: F, t2415: F, t7894: F, t15039: F, t2019: F, t2020: F, t2012: F, t8817: F) -> (F, F, F, F, F, F) {
    let t74247 = t16503 * t35039 * t665 * t9169;
    let t74249 = t34761 * t15405;
    let t74253 = t16503 * t34976 * t665 * t1502;
    let t74256 = t2010 * t2415 * t7894;
    let t74259 = t2019 * t2020 * t15039;
    let t74262 = t2010 * t2012 * t8817;
    (t74247, t74249, t74253, t74256, t74259, t74262)
}
