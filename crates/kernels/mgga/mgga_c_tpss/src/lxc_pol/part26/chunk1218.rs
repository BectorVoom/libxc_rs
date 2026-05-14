//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1218/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1218<F: Float>(t1338: F, t6054: F, t19261: F, t19649: F, t19651: F, t19653: F, t19655: F, t19658: F, t19660: F, t19662: F, t19664: F, t19666: F, t20706: F, t20786: F, t20957: F, t3537: F, t5986: F, t645: F) -> (F, F) {
    let t20969 = t6054 * t1338;
    let t20981 = 2.0 * t1338 * t19261 + 2.0 * t1338 * t20706 + 2.0 * t20957 * t645 + 2.0 * t3537 * t5986 + t19649 + t19651 + t19653 + t19655 + t19658 + t19660 + t19662 + t19664 + t19666 + t20786;
    (t20969, t20981)
}
