//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1207/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1207<F: Float>(t76132: F, t76167: F, t76193: F, t76227: F, t76259: F, t76295: F, t76333: F, t76394: F, t10080: F, t1499: F, t16673: F, t17027: F, t20857: F, t20858: F, t21014: F, t226: F, t235: F, t255: F, t40932: F, t4166: F, t46524: F, t5585: F, t5612: F, t5617: F, t5653: F, t59355: F, t76086: F, t76090: F, t76373: F, t812: F) -> (F, F) {
    let t76397 = t76132 + t76167 + t76193 + t76227 + t76259 + t76295 + t76333 + t76394;
    let t76414 = -36.0 * t10080 * t76090 * t812 - 6.0 * t17027 * t5612 * t812 - 6.0 * t17027 * t5617 * t812 - 24.0 * t20857 * t46524 * t812 + t226 * t235 * t76397 + 24.0 * t40932 * t76086 * t812 + 12.0 * t5585 * t59355 * t812 + 4.0 * t1499 * t21014 - 6.0 * t16673 * t5653 - 24.0 * t20858 * t4166 + t255 * t76373;
    (t76397, t76414)
}
