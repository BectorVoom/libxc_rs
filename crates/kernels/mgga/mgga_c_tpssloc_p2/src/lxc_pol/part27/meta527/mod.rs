//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta527<F: Float>(t225: F, t387: F, t4657: F, t345: F, t7569: F, t1921: F, t25749: F, t986: F, t7593: F, t990: F, t25705: F, t349: F, t1066: F, t1920: F, t23346: F, t23385: F, t23387: F, t23389: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t6687: F, t6771: F, t6776: F, t6816: F, t7554: F, t7566: F, t7600: F, t7625: F) -> (F, F, F, F, F, F, F, F) {
        let (t25766, t25767, t25778, t25784, t25785, t25789, t25791) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1936::<F>(t225, t387, t4657, t345, t7569, t1921, t25749, t986, t7593, t990, t25705, t349);
        let t25794 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1937::<F>(t1066, t1920, t23346, t23385, t23387, t23389, t25767, t25778, t25785, t25789, t25791, t3026, t3169, t388, t4557, t4660, t4665, t6687, t6771, t6776, t6816, t7554, t7566, t7600, t7625);
    (t25766, t25767, t25778, t25784, t25785, t25789, t25791, t25794)
}
