//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 761/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk761<F: Float>(t1294: F, t9905: F, t9892: F, t3684: F, t9467: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F, t1287: F, t2225: F) -> (F, F, F, F, F, F, F, F) {
    let t12103 = F::cast_from(0.35089341735807877242e1_f64) * t1294 * t9905;
    let t12105 = F::cast_from(0.51947577317044391277e2_f64) * t1294 * t9892;
    let t12109 = F::cast_from(0.21687162600603479684e-1_f64) * t3684 * t9467;
    let t12114 = F::cast_from(0.32530743900905219526e-1_f64) * t3684 * t9882;
    let t12116 = F::cast_from(0.48159733137676571078e0_f64) * t3684 * t9888;
    let t12118 = F::cast_from(0.16265371950452609763e-1_f64) * t3684 * t9885;
    let t12120 = t588 * t3824;
    let t12121 = F::new(12.0) * t12120;
    let t12123 = F::new(60.0) * t2225 * t1287;
    (t12103, t12105, t12109, t12114, t12116, t12118, t12121, t12123)
}
