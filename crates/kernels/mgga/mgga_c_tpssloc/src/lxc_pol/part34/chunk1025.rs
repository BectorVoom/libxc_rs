//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1025/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1025<F: Float>(t26959: F, t7428: F, t27979: F, t7032: F, t1860: F, t27956: F, t7031: F, t2031: F, t96461: F, t96469: F, t26016: F, t92047: F, t96425: F, t23967: F, t27972: F, t27976: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102137 = t7428 * t26959;
    let t102139 = t27979 * t7032;
    let t102142 = t1860 * t7031 * t27956;
    let t102163 = t2031 * t96461;
    let t102168 = t2031 * t96469;
    let t102173 = t26016 * t92047;
    let t102187 = t2031 * t96425;
    let t102192 = t23967 * t27972;
    let t102194 = t23967 * t27976;
    (t102137, t102139, t102142, t102163, t102168, t102173, t102187, t102192, t102194)
}
