//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 968/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk968<F: Float>(t3966: F, t751: F, t707: F, t157: F, t9897: F, t2371: F, t4199: F, t1409: F, t2517: F, t1484: F, t212: F, t9523: F) -> (F, F, F, F, F, F) {
    let t12932 = t751 * t3966;
    let t12934 = F::new(8.0) * t707 * t12932;
    let t12939 = t9897 * t157;
    let t12943 = t4199 * t2371;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    (t12934, t12939, t12943, t12946, t12984, t12985)
}
