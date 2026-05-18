//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 735/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk735<F: Float>(t2374: F, t9885: F, t2528: F, t677: F, t2509: F, t745: F, t9843: F, t761: F, t152: F, t31: F, t2368: F, t2505: F) -> (F, F, F, F, F, F, F) {
    let t9887 = F::new(0.16265371950452609763e-1) * t2374 * t9885;
    let t9888 = t677 * t2528;
    let t9890 = F::new(0.48159733137676571078e0) * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    let t9894 = F::new(0.51947577317044391277e2) * t761 * t9892;
    let t9897 = t31 * t152;
    let t9905 = t2368 * t745 * t2505;
    (t9887, t9888, t9890, t9892, t9894, t9897, t9905)
}
