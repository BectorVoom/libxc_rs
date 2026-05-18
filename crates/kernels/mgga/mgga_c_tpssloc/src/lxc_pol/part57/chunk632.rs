//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 632/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk632<F: Float>(t3: F, t7945: F, t1458: F, t2039: F, t1401: F, t3941: F, t5371: F, t577: F, t7230: F, t7801: F, t33: F, t68: F, t69: F) -> (F, F, F, F, F) {
    let t7946 = t3 * t7945;
    let t7956 = t2039 * t1458;
    let t7961 = F::new(0.45e1) * t7945 * t577 + F::new(0.135e2) * t7230 * t1458 + F::new(0.135e2) * t5371 * t2039 + F::new(27.0) * t3941 * t7956 + F::new(0.135e2) * t1401 * t7801;
    let t8301 = t33 * t33;
    let t8306 = F::new(1.0) / t69 / t68;
    (t7946, t7956, t7961, t8301, t8306)
}
