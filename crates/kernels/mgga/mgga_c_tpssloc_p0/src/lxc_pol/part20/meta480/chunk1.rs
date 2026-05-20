//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1962/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1962<F: Float>(t15979: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t12477: F, t15970: F, t15972: F, t15973: F, t15974: F, t15975: F, t15976: F, t15978: F, t1799: F, t3719: F, t3918: F, t5122: F, t9797: F, t9820: F, t9824: F) -> (F, F) {
    let t15980 = F::cast_from(0.24415263074675393405e-3_f64) * t15979;
    let t15981 = -F::new(3.0) * t12477 * t1799 * t3918 + F::new(3.0) * t3719 * t3918 * t5122 + t12103 - t12105 - t12109 - t12114 + t12116 + t12118 + t12123 + t15970 + t15972 + t15973 - t15974 + t15975 + t15976 + t15978 + t15980 + t9797 - t9820 - t9824;
    (t15980, t15981)
}
