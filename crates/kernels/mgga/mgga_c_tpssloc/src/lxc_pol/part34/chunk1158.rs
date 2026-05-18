//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1158/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1158<F: Float>(t2031: F, t96461: F, t96469: F, t26016: F, t92047: F, t96425: F, t23967: F, t27972: F, t27976: F, t27982: F, t7032: F, t26959: F, t7435: F) -> (F, F, F, F, F, F, F, F) {
    let t102163 = t2031 * t96461;
    let t102168 = t2031 * t96469;
    let t102173 = t26016 * t92047;
    let t102187 = t2031 * t96425;
    let t102192 = t23967 * t27972;
    let t102194 = t23967 * t27976;
    let t102215 = t27982 * t7032;
    let t102217 = t7435 * t26959;
    (t102163, t102168, t102173, t102187, t102192, t102194, t102215, t102217)
}
