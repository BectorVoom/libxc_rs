//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 707/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk707<F: Float>(t28: F, t528: F, t1081: F, t3231: F, t517: F, t157: F, t3671: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t3672 = F::new(1.0) / t528;
    let t3673 = t1081 * t1081;
    let t3679 = piecewise3::<f64>(t29, F::new(0.0), F::new(4.0) / F::new(9.0) * t3672 * t3673 + F::new(4.0) / F::new(3.0) * t517 * t3231);
    let t3681 = (t3671 + t3679) * t157;
    (t3672, t3673, t3681)
}
