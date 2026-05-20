//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta608<F: Float>(t90791: F, t90794: F, t90797: F, t90805: F, t90844: F, t90859: F, t90864: F, t90866: F, t90898: F, t90912: F, t90956: F, t90961: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93489, t93490, t93491, t93494, t93524, t93528, t93529, t93537, t93562, t93572, t93588, t93589) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1853::<F>(t90791, t90794, t90797, t90805, t90844, t90859, t90864, t90866, t90898, t90912, t90956, t90961);
    (t93489, t93490, t93491, t93494, t93524, t93528, t93529, t93537, t93562, t93572, t93588, t93589)
}
