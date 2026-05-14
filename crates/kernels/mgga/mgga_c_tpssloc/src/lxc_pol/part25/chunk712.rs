//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 712/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk712<F: Float>(t2374: F, t9882: F, t2535: F, t677: F, t2528: F, t2509: F, t745: F, t9843: F, t761: F, t2427: F, t2655: F, t152: F, t31: F, t185: F, t9288: F, t2448: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9884 = 0.32530743900905219526e-1 * t2374 * t9882;
    let t9885 = t677 * t2535;
    let t9887 = 0.16265371950452609763e-1 * t2374 * t9885;
    let t9888 = t677 * t2528;
    let t9890 = 0.48159733137676571078e0 * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    let t9894 = 0.51947577317044391277e2 * t761 * t9892;
    let t9896 = 12.0 * t2427 * t2655;
    let t9897 = t31 * t152;
    let t9898 = t185 * t9288;
    let t9900 = 24.0 * t9897 * t9898;
    let t9901 = t2448 * t67;
    (t9884, t9885, t9887, t9888, t9890, t9892, t9894, t9896, t9900, t9901)
}
