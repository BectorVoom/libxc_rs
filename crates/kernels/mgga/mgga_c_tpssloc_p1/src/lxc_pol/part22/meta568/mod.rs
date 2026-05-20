//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta568<F: Float>(t2402: F, t973: F, t986: F, t10213: F, t135: F, t41961: F, t697: F, t976: F, t984: F, t13797: F, t10216: F, t343: F, t10383: F, t964: F, t10868: F, t820: F, t1015: F, t10472: F, t42559: F, t204: F, t376: F, t1020: F, t1023: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42903, t42972, t43002, t43052, t43053, t43069, t43070) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074::<F>(t2402, t973, t986, t10213, t135, t41961, t697, t976, t984, t13797, t10216, t343);
        let (t43157, t43198, t43211, t43216, t43219) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075::<F>(t10383, t964, t10868, t820, t1015, t10472, t42559, t204, t376, t1020, t1023, t248);
    (t42903, t42972, t43002, t43052, t43053, t43069, t43070, t43157, t43198, t43211, t43216, t43219)
}
