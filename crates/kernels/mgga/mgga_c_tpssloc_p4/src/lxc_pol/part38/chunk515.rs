//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 515/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk515<F: Float>(t109: F, t532: F, t556: F, t656: F, t91: F, t96: F, t64: F) -> (F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t1995 = F::cast_from(1.0_f64) / t556 / t532;
    let t2176 = t656 * t91;
    let t2177 = t2176 * t96;
    let t2180 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t64 * t2177 / F::cast_from(8.0_f64));
    (t1995, t2176, t2177, t2180)
}
