//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1201/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1201<F: Float>(t2363: F, t24932: F, t27888: F, t671: F, t7266: F, t83946: F, t83948: F, t83952: F, t83956: F, t83958: F, t83960: F, t83962: F, t83964: F, t83966: F, t83968: F, t85428: F, t85573: F, t85577: F, t9416: F) -> (F,) {
    let t85613 = 6.0 * t2363 * t24932 + 6.0 * t2363 * t27888 + 6.0 * t671 * t85428 + 2.0 * t7266 * t9416 + t83946 + t83948 + t83952 + t83956 + t83958 + t83960 + t83962 + t83964 + t83966 + t83968 + t85573 + 6.0 * t85577;
    (t85613,)
}
