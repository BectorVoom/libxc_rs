//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1285/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1285<F: Float>(t46104: F, t8662: F, t12571: F, t31867: F, t33676: F, t9231: F, t2240: F, t27363: F, t8301: F, t31863: F, t116114: F, t39063: F, t45844: F, t9239: F, t191: F, t192: F, t27903: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122952 = t46104 * t8662;
    let t122955 = t12571 * t31867;
    let t122960 = t9231 * t33676;
    let t122964 = t2240 * t8301 * t27363;
    let t122976 = t12571 * t31863;
    let t122979 = t39063 * t116114;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123111 = t27903 * t191 * t192;
    (t122952, t122955, t122960, t122964, t122976, t122979, t122988, t123001, t123111)
}
