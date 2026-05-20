//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1024;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta298<F: Float>(t21444: F, t340: F, t343: F, t974: F, t1597: F, t5836: F, t4546: F, t5842: F, t20217: F, t978: F, t977: F, t10217: F, t20234: F, t10214: F, t2980: F, t21126: F, t4518: F, t13909: F, t17784: F, t17809: F, t21430: F, t21433: F, t2986: F, t973: F) -> (F, F, F, F, F, F, F, F) {
        let (t21446, t21447, t21452, t21453, t21456, t21458, t21459, t21462, t21463, t21468) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1024::<F>(t21444, t340, t343, t974, t1597, t5836, t4546, t5842, t20217, t978, t977, t10217, t20234);
        let (t21472, t21479) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1025::<F>(t10214, t21468, t20234, t2980, t977, t21126, t4518, t13909, t17784, t17809, t21430, t21433, t21447, t21453, t21459, t21463, t2986, t973);
    (t21446, t21452, t21456, t21458, t21462, t21468, t21472, t21479)
}
