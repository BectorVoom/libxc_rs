//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 885/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk885<F: Float>(t4483: F, t5812: F, t1568: F, t5742: F, t2888: F, t10277: F, t20234: F, t2826: F, t136: F, t4337: F, t5398: F, t2768: F) -> (F, F, F, F, F, F, F) {
    let t21107 = F::cast_from(0.51947577317044391276e2_f64) * t4483 * t5812;
    let t21114 = t5742 * t1568;
    let t21115 = t21114 * t2888;
    let t21118 = t10277 * t20234;
    let t21119 = t2826 * t21118;
    let t21120 = t136 * t21119;
    let t21122 = t4337 * t5398;
    let t21123 = t2768 * t21122;
    (t21107, t21114, t21115, t21118, t21120, t21122, t21123)
}
