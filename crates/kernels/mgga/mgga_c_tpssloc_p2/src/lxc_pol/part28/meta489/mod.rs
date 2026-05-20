//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta489<F: Float>(t1339: F, t26297: F, t22827: F, t1307: F, t1825: F, t22833: F, t5259: F, t22759: F, t242: F, t1336: F) -> (F, F, F, F, F, F, F, F) {
        let (t26298, t26299, t26301, t26302, t26303, t26306, t26308, t26309) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1703::<F>(t1339, t26297, t22827, t1307, t1825, t22833, t5259, t22759, t242, t1336);
    (t26298, t26299, t26301, t26302, t26303, t26306, t26308, t26309)
}
