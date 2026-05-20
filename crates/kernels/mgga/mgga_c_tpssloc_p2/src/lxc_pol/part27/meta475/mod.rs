//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta475<F: Float>(t3014: F, t343: F, t6734: F, t1004: F, t6758: F, t1036: F, t6750: F, t1940: F, t3087: F, t354: F, t6759: F, t3: F, t6740: F, t23476: F, t1046: F, t1935: F, t23533: F, t23537: F, t23541: F, t23544: F, t3043: F, t3134: F, t3153: F, t378: F, t6717: F, t6747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844::<F>(t3014, t343, t6734, t1004, t6758, t1036, t6750, t1940, t3087, t354, t6759, t3, t6740);
        let (t23563, t23564, t23569) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845::<F>(t23476, t343, t23562, t1046, t1935, t23533, t23537, t23541, t23544, t23548, t23551, t23554, t23557, t23560, t3043, t3134, t3153, t378, t6717, t6747);
    (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562, t23563, t23564, t23569)
}
