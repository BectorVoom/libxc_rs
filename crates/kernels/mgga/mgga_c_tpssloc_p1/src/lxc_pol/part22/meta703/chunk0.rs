//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2290/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290<F: Float>(t15689: F, t4889: F, t1174: F, t135: F, t18996: F, t15743: F, t5024: F, t18363: F, t3577: F, t45124: F, t11697: F, t18359: F) -> (F, F, F, F, F) {
    let t66273 = t4889 * t15689;
    let t66276 = t1174 * t135 * t18996;
    let t66324 = t5024 * t15743;
    let t66334 = t3577 * t45124 * t18363;
    let t66337 = t3577 * t11697 * t18359;
    (t66273, t66276, t66324, t66334, t66337)
}
