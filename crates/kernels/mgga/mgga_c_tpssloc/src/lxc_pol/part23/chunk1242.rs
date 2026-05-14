//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1242/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1242<F: Float>(t42086: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F, t77072: F, t894: F) -> (F, F) {
    let t77097 = -0.98587999999999999998e0 * t76893 + 0.43816888888888888889e0 * t76896 + 0.197176e1 * t76909 + 0.49293999999999999999e0 * t76915 - 0.88582716049382716048e0 * t76574 - 0.29896666666666666667e0 * t76578 + 0.39862222222222222223e1 * t76583 - 0.71752000000000000002e1 * t76591 - 0.59793333333333333333e0 * t76599 + 0.71752e1 * t76614 + 0.17938e1 * t76622 + 0.15944888888888888889e1 * t59688 - 0.79724444444444444446e0 * t59694 + t42086;
    let t77102 = t894 * t77072;
    (t77097, t77102)
}
