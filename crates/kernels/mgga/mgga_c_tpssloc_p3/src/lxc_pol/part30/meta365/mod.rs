//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1410;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta365<F: Float>(t248: F, t3521: F, t4733: F, t1227: F, t3536: F, t4997: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F, t3503: F, t3500: F, t1210: F, t11539: F, t4724: F, t1174: F, t13969: F, t4983: F, t3515: F, t478: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15488, t15490, t15494, t15495, t15498) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1410::<F>(t248, t3521, t4733, t1227, t3536, t4997, t3570, t5012, t1213, t3535, t5018, t1202, t5023);
        let (t15503, t15507, t15524, t15550, t15567) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1411::<F>(t1742, t3036, t3503, t3500, t1210, t11539, t4724, t1174, t13969, t4983, t3515, t478);
    (t15488, t15490, t15494, t15495, t15498, t15503, t15507, t15524, t15550, t15567)
}
