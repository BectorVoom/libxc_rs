//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 654/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk654<F: Float>(t111: F, t2022: F, t22468: F, t2094: F, t531: F, t7025: F, t9239: F, t33: F, t625: F, t2240: F, t240: F, t67: F, t1864: F, t1860: F, t22819: F, t22825: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23880 = t2022 * t111;
    let t23912 = 22.0 / 9.0 * t22468;
    let t23957 = t531 * t2094;
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = 88.0 / 27.0 * t1860 * t23993;
    let t24049 = 0.33643963411783659044e-4 * t22819;
    let t24050 = 0.10541775202358879834e-2 * t22825;
    (t23880, t23912, t23957, t23963, t23966, t23967, t23995, t24049, t24050)
}
