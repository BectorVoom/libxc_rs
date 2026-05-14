//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1235/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1235<F: Float>(t24977: F, t576: F, t1395: F, t7426: F, t12513: F, t12537: F, t1396: F, t1398: F, t1404: F, t2170: F, t2174: F, t24955: F, t3: F, t3932: F, t3946: F, t580: F, t7416: F, t85403: F, t85405: F, t85407: F, t85412: F, t86550: F, t86553: F) -> (F,) {
    let t86557 = t576 * t24977;
    let t86559 = t1395 * t7426;
    let tv4rho3sigma2 = t3 * t580 * t86550 + t12513 * t2174 + t12537 * t2170 + 3.0 * t1396 * t24977 + t1398 * t86553 + 3.0 * t1404 * t24955 + 3.0 * t3932 * t7426 + 3.0 * t3946 * t7416 + 3.0 * t85403 + 6.0 * t85405 + 3.0 * t85407 + 3.0 * t85412 + 3.0 * t86557 + 6.0 * t86559;
    (tv4rho3sigma2,)
}
