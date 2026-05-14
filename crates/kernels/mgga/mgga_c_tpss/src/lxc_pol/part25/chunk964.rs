//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 964/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk964<F: Float>(t14256: F, t2389: F, t774: F, t14003: F, t14034: F, t14036: F, t14050: F, t14053: F, t14057: F, t7929: F, t7932: F, t7936: F, t8000: F, t8019: F, t8023: F, t8024: F, t8029: F, t8040: F) -> (F, F) {
    let t14258 = t2389 * t774 * t14256;
    let t14261 = t8000 - t14003 - t8019 + t8023 + t8024 + t14034 - t8029 + t14036 - t8040 + t14050 - t14053 + t7929 - t7932 - t7936 - t14057;
    (t14258, t14261)
}
