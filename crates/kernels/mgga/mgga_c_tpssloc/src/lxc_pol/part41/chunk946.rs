//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 946/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk946<F: Float>(t12922: F, t12926: F, t12934: F, t16612: F, t16618: F, t16622: F, t16623: F, t16624: F, t16625: F, t16629: F, t16631: F, t16633: F, t16636: F, t16662: F, t193: F, t2522: F, t4255: F, t4310: F, t4314: F, t766: F, t776: F, t9715: F, t9724: F, t9726: F, t9780: F, t9863: F) -> (F,) {
    let t16666 = -3.0 * t16625 * t2522 * t776 + 3.0 * t16662 * t193 * t766 + 12.0 * t4255 * t4310 * t4314 + t12922 + t12926 + t12934 + t16612 - t16618 + t16622 + t16623 - t16624 + t16629 + t16631 + t16633 + t16636 - t9715 + t9724 + t9726 + t9780 + t9863;
    (t16666,)
}
