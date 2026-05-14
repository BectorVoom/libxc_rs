//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 785/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk785<F: Float>(t20173: F, t8657: F, t1873: F, t7056: F, t3941: F, t2039: F, t6534: F, t191: F, t192: F, t7412: F, t8662: F, t9231: F, t9239: F, t131: F, t7245: F, t2240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31813 = 27.0 * t20173 * t8657;
    let t31814 = t7056 * t1873;
    let t31816 = 27.0 * t3941 * t31814;
    let t31817 = t2039 * t6534;
    let t31819 = 27.0 * t3941 * t31817;
    let t31832 = t7412 * t191 * t192;
    let t31857 = t9231 * t8662;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    (t31813, t31814, t31816, t31817, t31819, t31832, t31857, t31860, t31863, t31864)
}
