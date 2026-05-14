//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1347/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1347<F: Float>(t1338: F, t13546: F, t19261: F, t20706: F, t20957: F, t25354: F, t3537: F, t4674: F, t5986: F, t645: F, t68156: F, t69080: F, t69082: F, t69084: F, t69086: F, t72774: F, t72781: F, t73086: F, t73089: F) -> (F,) {
    let t73114 = 4.0 * t1338 * t68156 + 4.0 * t1338 * t72781 + 2.0 * t13546 * t5986 + 2.0 * t19261 * t4674 + 2.0 * t20706 * t4674 + 4.0 * t20957 * t3537 + 4.0 * t25354 * t3537 + 2.0 * t645 * t72774 + t69080 + t69082 + t69084 + t69086 + t73086 + 2.0 * t73089;
    (t73114,)
}
