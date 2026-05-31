//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 987/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk987<F: Float>(t5: F, t12722: F, t112: F, t111: F, t4025: F, t1441: F, t2319: F, t649: F, t671: F, t2363: F, t88: F, t1454: F, t2281: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t12723 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t12722);
    let t12724 = t12723 * t112;
    let t12725 = t4025 * t111;
    let t12728 = t1441 * t2319;
    let t12734 = t649 * t671;
    let t12739 = t88 * t2363;
    let t12747 = t2281 * t1454;
    (t12724, t12725, t12728, t12734, t12739, t12747)
}
