//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1112/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1112<F: Float>(t12078: F, t12175: F, t12217: F, t12273: F, t294: F, t1151: F, t11821: F, t11823: F, t11828: F, t11832: F, t11836: F, t11839: F, t11970: F, t11973: F, t11975: F, t11978: F, t11980: F, t11982: F, t12002: F, t12004: F, t12006: F, t12008: F, t12011: F, t12012: F, t4023: F) -> (F, F) {
    let t12276 = t294 * (t12078 + t12175 + t12217 + t12273);
    let t12277 = -F::new(2.0) * t1151 * t12012 * t4023 - t11821 + t11823 - t11828 + t11832 - t11836 - t11839 + t11970 + t11973 + t11975 + t11978 + t11980 + t11982 + t12002 - t12004 + t12006 - t12008 - t12011 + t12276;
    (t12276, t12277)
}
