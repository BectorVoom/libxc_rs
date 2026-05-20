//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2161/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161<F: Float>(t53984: F, t40281: F, t5303: F, t5247: F, t820: F, t12250: F, t1824: F, t16060: F, t3789: F, t12384: F, t5234: F, t5293: F) -> (F, F, F, F, F, F, F) {
    let t53985 = F::new(35.0) / F::new(72.0) * t53984;
    let t53997 = t40281 * t5303;
    let t53998 = F::new(119.0) / F::new(1152.0) * t53997;
    let t54013 = t5247 * t820;
    let t54014 = t1824 * t12250;
    let t54023 = t16060 * t3789;
    let t54042 = t5234 * t12384;
    let t54047 = t40281 * t5293;
    (t53985, t53998, t54013, t54014, t54023, t54042, t54047)
}
