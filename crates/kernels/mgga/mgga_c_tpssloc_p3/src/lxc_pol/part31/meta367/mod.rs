//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1298;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1299;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta367<F: Float>(t4234: F, t4295: F, t12850: F, t12860: F, t16577: F, t16578: F, t16581: F, t16582: F, t16588: F, t16612: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F, t12946: F, t12922: F, t12926: F, t12934: F, t16618: F, t16622: F, t16623: F, t16624: F, t16629: F, t16631: F, t16633: F, t16636: F, t9726: F, t9780: F, t9789: F, t9863: F, t9869: F, t5519: F, t706: F, t708: F, t9871: F, t13115: F, t157: F, t4196: F, t9880: F, t13107: F, t13105: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16679, t16684) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1298::<F>(t4234, t4295, t12850, t12860, t16577, t16578, t16581, t16582, t16588, t16612, t9457, t9469, t9476, t9484, t9496, t9715, t9724);
        let (t16685, t16686) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1299::<F>(t12946, t12922, t12926, t12934, t16618, t16622, t16623, t16624, t16629, t16631, t16633, t16636, t9726, t9780, t9789, t9863);
        let (t16688, t16691, t16692, t16695, t16696, t16697, t16698) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1300::<F>(t9869, t5519, t706, t708, t9871, t13115, t157, t4196, t9880, t13107, t13105, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
    (t16679, t16684, t16685, t16686, t16688, t16691, t16692, t16695, t16696, t16697, t16698)
}
