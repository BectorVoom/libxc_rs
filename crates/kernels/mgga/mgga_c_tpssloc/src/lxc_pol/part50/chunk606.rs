//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 606/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk606<F: Float>(t3: F, t5363: F, t112: F, t1851: F, t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t577: F, t2235: F, t33: F, t1862: F) -> (F, F, F, F, F, F) {
    let t5364 = t3 * t5363;
    let t5371 = t1851 * t112;
    let t5376 = t1458 * t671;
    let t5381 = 0.45e1 * t5363 * t577 + 0.135e2 * t5371 * t671 + 0.135e2 * t3938 * t1458 + 27.0 * t3941 * t5376 + 0.135e2 * t1401 * t4072;
    let t6486 = t2235 * t33;
    let t6489 = t33 * t1862;
    (t5364, t5371, t5376, t5381, t6486, t6489)
}
