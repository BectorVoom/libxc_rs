//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1836;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta563<F: Float>(t1888: F, t23270: F, t25044: F, t2742: F, t23168: F, t25342: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t22986: F, t23272: F, t2379: F, t25038: F, t25053: F, t25054: F, t82159: F, t25229: F, t23222: F, t25224: F, t6552: F) -> (F, F, F, F, F, F, F, F) {
        let (t86866, t86868, t86870, t86875) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1836::<F>(t1888, t23270, t25044, t2742, t23168, t25342, t25345, t82038, t1519, t213, t225, t22986, t23272);
        let (t86881, t86884, t86886, t86891) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1837::<F>(t23270, t2379, t25038, t25053, t22986, t25054, t82159, t23168, t25229, t23222, t25224, t6552);
    (t86866, t86868, t86870, t86875, t86881, t86884, t86886, t86891)
}
