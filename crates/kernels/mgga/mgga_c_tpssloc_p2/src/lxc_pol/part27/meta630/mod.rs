//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2119;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2120;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta630<F: Float>(t1877: F, t2219: F, t6666: F, t25353: F, t2752: F, t25213: F, t6547: F, t22986: F, t23270: F, t25053: F, t2553: F, t4119: F, t857: F, t865: F, t4300: F, t776: F, t1888: F, t2717: F, t25044: F, t2742: F, t23168: F, t25342: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t23272: F, t2379: F, t25038: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86835, t86836, t86844, t86847, t86849) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2119::<F>(t1877, t2219, t6666, t25353, t2752, t25213, t6547, t22986, t23270, t25053, t2553, t4119, t857);
        let (t86852, t86857, t86862, t86866, t86868) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2120::<F>(t22986, t23270, t865, t86849, t4300, t776, t857, t1888, t2717, t25044, t2742, t23168, t25342);
        let (t86869, t86870, t86875, t86881) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2121::<F>(t86868, t25345, t82038, t1519, t213, t225, t22986, t23272, t23270, t2379, t25038, t25053);
    (t86835, t86836, t86844, t86847, t86852, t86857, t86862, t86866, t86869, t86870, t86875, t86881)
}
