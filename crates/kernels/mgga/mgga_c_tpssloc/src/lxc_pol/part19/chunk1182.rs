//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1182/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182<F: Float>(t2794: F, t2836: F, t2842: F, t2784: F, t2791: F, t2897: F, t2929: F, t10629: F, t938: F, t2903: F, t2928: F, t315: F, t2906: F) -> (F, F, F, F, F, F, F) {
    let t41804 = 36.0 * t2842 * t2794 * t2836;
    let t41811 = t2784 * t2791;
    let t41813 = 12.0 * t41811 * t2794;
    let t41816 = t2897 * t2929;
    let t41821 = t938 * t10629;
    let t41825 = 1.0 / t2928 / t2903;
    let t41826 = t315 * t41825;
    let t41827 = t2906 * t2906;
    (t41804, t41813, t41816, t41821, t41825, t41826, t41827)
}
