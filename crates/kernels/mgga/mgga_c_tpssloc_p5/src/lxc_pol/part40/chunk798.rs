//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 798/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk798<F: Float>(t1499: F, t1523: F, t1525: F, t226: F, t255: F, t4166: F, t5575: F, t5645: F, t5648: F, t5651: F, t5653: F, t5655: F, t812: F) -> F {
    let t5657 = F::cast_from(2.0_f64) * t1499 * t1525 - F::cast_from(2.0_f64) * t1523 * t4166 + t226 * t5655 + t255 * t5575 + F::cast_from(2.0_f64) * t5645 * t812 - F::cast_from(2.0_f64) * t5648 * t812 - t5651 * t812 - t5653 * t812;
    t5657
}
