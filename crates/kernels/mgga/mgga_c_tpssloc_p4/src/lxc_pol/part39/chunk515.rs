//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 515/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk515<F: Float>(t109: F, t532: F, t556: F, t656: F, t99: F, t104: F, t64: F) -> (F, F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t1995 = F::new(1.0) / t556 / t532;
    let t2195 = t656 * t99;
    let t2196 = t2195 * t104;
    let t2199 = piecewise3::<F>(t110, F::new(0.0), -t64 * t2196 / F::new(8.0));
    (t1995, t2195, t2196, t2199)
}
