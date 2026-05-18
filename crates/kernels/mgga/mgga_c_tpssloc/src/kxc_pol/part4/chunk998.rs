//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 998/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk998<F: Float>(t12946: F, t12922: F, t12926: F, t12934: F, t16618: F, t16622: F, t16623: F, t16624: F, t16629: F, t16631: F, t16633: F, t16636: F, t9726: F, t9780: F, t9789: F, t9863: F) -> (F, F) {
    let t16685 = F::new(8.0) * t12946;
    let t16686 = t9726 + t9863 + t9780 - t16618 + t16622 + t12922 + t12926 + t16623 - t16624 + t12934 + t16629 + t16631 + t16633 + t16636 + t16685 - t9789;
    (t16685, t16686)
}
