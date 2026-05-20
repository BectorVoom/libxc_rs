//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1207/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1207<F: Float>(t6805: F, t986: F, t1949: F, t3016: F, t1022: F, t6768: F, t1060: F, t6733: F, t6743: F, t6801: F, t1945: F, t3040: F) -> (F, F, F, F, F, F) {
    let t23647 = t986 * t6805;
    let t23650 = t3016 * t1949;
    let t23653 = t6768 * t1022;
    let t23654 = t23653 * t1060;
    let t23657 = t6733 * t6743;
    let t23658 = t23657 * t6801;
    let t23661 = t1945 * t3040;
    (t23647, t23650, t23654, t23657, t23658, t23661)
}
