//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2073/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073<F: Float>(t7288: F, t85660: F, t225: F, t24758: F, t24637: F, t7294: F, t2121: F, t3427: F, t7295: F, t24901: F, t3640: F, t11947: F, t7394: F) -> (F, F, F, F, F, F) {
    let t86473 = t85660 * t7288;
    let t86475 = t24758 * t225;
    let t86494 = t7294 * t24637;
    let t86501 = t2121 * t3427 * t7295;
    let t86513 = t24901 * t3640;
    let t86517 = t7394 * t11947;
    (t86473, t86475, t86494, t86501, t86513, t86517)
}
