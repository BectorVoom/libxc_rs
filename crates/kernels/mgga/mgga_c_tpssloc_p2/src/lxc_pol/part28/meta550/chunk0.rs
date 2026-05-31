//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1820/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1820<F: Float>(t111: F, t7222: F, t81437: F, t112: F, t24447: F, t24007: F, t22550: F, t7031: F, t22549: F, t2031: F, t83728: F, t83737: F) -> (F, F, F, F, F, F, F, F) {
    let t84033 = t7222 * t111;
    let t84036 = F::cast_from(308.0_f64) / F::cast_from(27.0_f64) * t81437;
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    let t84173 = t7031 * t22550;
    let t84174 = t22549 * t84173;
    let t84180 = t2031 * t83728;
    let t84183 = t2031 * t83737;
    (t84033, t84036, t84078, t84097, t84173, t84174, t84180, t84183)
}
