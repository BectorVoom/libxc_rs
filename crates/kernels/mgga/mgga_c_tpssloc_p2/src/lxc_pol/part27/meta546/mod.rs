//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta546<F: Float>(t16225: F, t550: F, t1339: F, t22827: F, t1307: F, t1825: F, t22833: F, t5259: F, t22759: F, t242: F, t1336: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26297, t26298, t26299, t26301, t26302, t26303, t26306, t26308, t26309) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1976::<F>(t16225, t550, t1339, t22827, t1307, t1825, t22833, t5259, t22759, t242, t1336);
    (t26297, t26298, t26299, t26301, t26302, t26303, t26306, t26308, t26309)
}
