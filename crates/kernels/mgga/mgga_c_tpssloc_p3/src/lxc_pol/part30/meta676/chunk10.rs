//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2117/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117<F: Float>(t1873: F, t96657: F, t28007: F, t6534: F, t26114: F, t7467: F, t26117: F, t26135: F, t7676: F, t2314: F, t28017: F, t5113: F) -> (F, F, F, F, F, F, F) {
    let t96659 = F::new(2.0) * t96657 * t1873;
    let t96661 = F::new(2.0) * t28007 * t6534;
    let t96663 = F::new(4.0) * t26114 * t7467;
    let t96665 = F::new(4.0) * t26117 * t7467;
    let t96667 = F::new(4.0) * t7676 * t26135;
    let t96669 = F::new(2.0) * t2314 * t28017;
    let t96671 = F::new(2.0) * t5113 * t28017;
    (t96659, t96661, t96663, t96665, t96667, t96669, t96671)
}
