//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1079/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1079<F: Float>(t1845: F, t6995: F, t26161: F, t26162: F, t31537: F, t7468: F, t31540: F, t26003: F, t8526: F, t24995: F, t37593: F, t5308: F, t5107: F, t8320: F, t1774: F, t30991: F) -> (F, F, F, F, F, F, F) {
    let t119832 = t1845 * t6995;
    let t119835 = 4.0 * t26161 * t26162 * t119832;
    let t119837 = 4.0 * t31537 * t7468;
    let t119839 = 4.0 * t31540 * t7468;
    let t119841 = 4.0 * t8526 * t26003;
    let t119844 = 6.0 * t24995 * t37593 * t5308;
    let t119850 = 2.0 * t8320 * t5107;
    let t119852 = 2.0 * t30991 * t1774;
    (t119835, t119837, t119839, t119841, t119844, t119850, t119852)
}
