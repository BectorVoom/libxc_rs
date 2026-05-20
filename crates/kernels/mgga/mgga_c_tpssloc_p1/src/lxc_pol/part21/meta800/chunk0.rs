//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2787/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2787<F: Float>(t118: F, t2375: F, t5522: F, t46335: F, t46348: F, t16575: F, t706: F, t708: F, t46369: F, t46371: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t40801: F, t40803: F, t58060: F, t58061: F, t58062: F, t58080: F, t58085: F, t58094: F) -> (F, F, F, F, F, F, F) {
    let t58972 = t5522 * t118 * t2375;
    let t58973 = F::cast_from(0.10843581300301739842e-1_f64) * t58972;
    let t58974 = F::new(16.0) * t46335;
    let t58975 = F::new(48.0) * t46348;
    let t58976 = t706 * t16575;
    let t58978 = F::new(8.0) * t58976 * t708;
    let t58979 = F::new(8.0) * t46369;
    let t58980 = F::cast_from(0.43374325201206959368e-1_f64) * t46371;
    let t58981 = t40801 - t40803 - t58060 + t58061 + t58062 + t39549 + t58080 + t39563 + t58085 - t58094 + t58973 - t39585 + t39590 + t58974 - t39593 + t58975 + t58978 + t58979 - t58980;
    (t58973, t58974, t58975, t58978, t58979, t58980, t58981)
}
