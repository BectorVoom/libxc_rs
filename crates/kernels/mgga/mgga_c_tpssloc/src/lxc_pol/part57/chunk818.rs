//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 818/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk818<F: Float>(t29025: F, t29039: F, t235: F, t5617: F, t7101: F, t1499: F, t2051: F, t226: F, t24265: F, t25277: F, t25293: F, t25310: F, t25317: F, t28420: F, t28424: F, t28428: F, t29010: F, t5575: F, t7839: F, t812: F) -> (F, F) {
    let t29040 = t29025 + t29039;
    let t29041 = t235 * t29040;
    let t29052 = t7101 * t5617;
    let t29054 = -t812 * t29010 - t24265 + F::new(0.76763589786250567036e-1) * t25277 + t226 * t29041 + F::new(2.0) * t1499 * t7839 - F::new(0.76763589786250567036e-1) * t25293 + t5575 * t2051 + F::new(0.15352717957250113407e0) * t25310 + F::new(0.3289868133696452873e-1) * t25317 - F::new(0.3289868133696452873e-1) * t28420 - F::new(0.16449340668482264365e-1) * t28424 + F::new(0.3289868133696452873e-1) * t28428 - t812 * t29052;
    (t29040, t29054)
}
