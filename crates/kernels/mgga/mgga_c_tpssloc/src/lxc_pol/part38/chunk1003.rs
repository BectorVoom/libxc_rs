//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1003/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1003<F: Float>(t1294: F, t9905: F, t9892: F, t3684: F, t9467: F, t118: F, t1284: F, t2375: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F) -> (F, F, F, F, F, F, F, F) {
    let t12103 = F::new(0.35089341735807877242e1) * t1294 * t9905;
    let t12105 = F::new(0.51947577317044391277e2) * t1294 * t9892;
    let t12109 = F::new(0.21687162600603479684e-1) * t3684 * t9467;
    let t12110 = t1284 * t118;
    let t12111 = t12110 * t2375;
    let t12114 = F::new(0.32530743900905219526e-1) * t3684 * t9882;
    let t12116 = F::new(0.48159733137676571078e0) * t3684 * t9888;
    let t12118 = F::new(0.16265371950452609763e-1) * t3684 * t9885;
    let t12120 = t588 * t3824;
    (t12103, t12105, t12109, t12111, t12114, t12116, t12118, t12120)
}
