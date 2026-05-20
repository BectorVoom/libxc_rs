//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2440/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2440<F: Float>(t14137: F, t3048: F, t10952: F, t13970: F, t13969: F, t14098: F, t3039: F, t10224: F, t4343: F, t973: F, t3130: F, t4595: F, t49850: F) -> (F, F, F, F, F) {
    let t49892 = t3048 * t14137;
    let t49894 = t10952 * t13970;
    let t49897 = t3039 * t13969 * t14098;
    let t49906 = t973 * t10224 * t4343;
    let t49907 = t49906 / F::new(216.0);
    let t49922 = t3130 * t49850 * t4595;
    (t49892, t49894, t49897, t49907, t49922)
}
