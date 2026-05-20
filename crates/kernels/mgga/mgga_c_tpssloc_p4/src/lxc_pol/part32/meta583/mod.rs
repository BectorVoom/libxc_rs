//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta583<F: Float>(t29726: F, t462: F, t1409: F, t1734: F, t7376: F, t24851: F, t1653: F, t27460: F, t7362: F, t6260: F, t7375: F, t1244: F, t2121: F, t2149: F, t24773: F, t24849: F, t27406: F, t27451: F, t27556: F, t29678: F, t29702: F, t29705: F, t29709: F, t29712: F, t29716: F, t29720: F, t29723: F, t3610: F, t3624: F, t5064: F, t7283: F, t7373: F, t8070: F, t8083: F) -> (F, F, F, F, F, F, F, F) {
        let (t29734, t29735, t29736, t29740, t29741, t29744, t29745, t29748) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1966::<F>(t29726, t462, t1409, t1734, t7376, t24851, t1653, t27460, t7362, t6260, t7375, t1244, t2121, t2149, t24773, t24849, t27406, t27451, t27556, t29678, t29702, t29705, t29709, t29712, t29716, t29720, t29723, t3610, t3624, t5064, t7283, t7373, t8070, t8083);
    (t29734, t29735, t29736, t29740, t29741, t29744, t29745, t29748)
}
