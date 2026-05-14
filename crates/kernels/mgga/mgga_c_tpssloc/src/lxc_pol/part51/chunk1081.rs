//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1081/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1081<F: Float>(t2039: F, t6862: F, t31531: F, t31532: F, t31539: F, t31542: F, t31544: F, t31548: F, t31671: F, t31722: F, t4034: F, t574: F, t6517: F, t652: F, t672: F, t7057: F, t7061: F, t7171: F, t8329: F, t8450: F, t8529: F) -> (F, F) {
    let t31726 = t6862 * t2039;
    let t31729 = -2.0 * t31532 * t672 + t31722 * t574 - 2.0 * t31726 * t652 - 2.0 * t4034 * t8529 - 2.0 * t6517 * t7057 - 2.0 * t6517 * t7061 + 3.0 * t7171 * t8450 - t31531 - t31539 - t31542 - t31544 - t31548 + t31671 - t8329;
    (t31726, t31729)
}
