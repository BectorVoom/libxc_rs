//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1140/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1140<F: Float>(t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t15970: F, t15972: F, t15973: F, t15974: F, t15975: F, t15976: F, t15978: F, t9820: F, t9824: F) -> (F,) {
    let t16163 = -t9820 - t9824 + t15970 + t15972 + t15973 - t15974 + t12103 - t12105 - t12109 + t15975 - t12114 + t12116 + t12118 + t15976 + t12123 + t15978;
    (t16163,)
}
