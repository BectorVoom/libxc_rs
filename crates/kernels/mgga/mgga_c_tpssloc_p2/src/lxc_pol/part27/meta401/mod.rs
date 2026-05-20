//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1670;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta401<F: Float>(t11975: F, t11977: F, t11981: F, t2528: F, t5154: F, t172: F, t5151: F, t763: F, t2535: F, t5166: F, t592: F, t12461: F, t1845: F, t11984: F, t1307: F, t1388: F, t15868: F, t15872: F, t15876: F, t15878: F, t15880: F, t15883: F, t3698: F, t3914: F, t5126: F, t5160: F, t5161: F, t9457: F, t9476: F, t9484: F, t9780: F) -> (F, F, F, F, F, F, F, F) {
        let (t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15899) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1670::<F>(t11975, t11977, t11981, t2528, t5154, t172, t5151, t763, t2535, t5166, t592, t12461, t1845);
        let t15903 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1671::<F>(t11984, t1307, t1388, t15868, t15872, t15876, t15878, t15880, t15883, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15899, t3698, t3914, t5126, t5160, t5161, t9457, t9476, t9484, t9780);
    (t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15903)
}
