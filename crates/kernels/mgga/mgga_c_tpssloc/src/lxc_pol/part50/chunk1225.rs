//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1225/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1225<F: Float>(t26003: F, t8526: F, t24995: F, t37593: F, t5308: F, t31246: F, t7756: F, t5107: F, t8320: F, t1774: F, t30991: F, t1799: F, t6995: F) -> (F, F, F, F, F, F) {
    let t119841 = F::new(4.0) * t8526 * t26003;
    let t119844 = F::new(6.0) * t24995 * t37593 * t5308;
    let t119845 = t31246 * t7756;
    let t119850 = F::new(2.0) * t8320 * t5107;
    let t119852 = F::new(2.0) * t30991 * t1774;
    let t119853 = t1799 * t6995;
    (t119841, t119844, t119845, t119850, t119852, t119853)
}
