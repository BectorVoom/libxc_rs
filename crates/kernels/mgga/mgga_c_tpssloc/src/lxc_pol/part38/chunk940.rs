//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 940/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk940<F: Float>(t2374: F, t9888: F, t2509: F, t745: F, t9843: F, t761: F, t152: F, t31: F, t2448: F, t67: F, t758: F, t2368: F, t2505: F) -> (F, F, F, F, F, F) {
    let t9890 = F::new(0.48159733137676571078e0) * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    let t9894 = F::new(0.51947577317044391277e2) * t761 * t9892;
    let t9897 = t31 * t152;
    let t9901 = t2448 * t67;
    let t9902 = t9901 * t758;
    let t9905 = t2368 * t745 * t2505;
    (t9890, t9892, t9894, t9897, t9902, t9905)
}
