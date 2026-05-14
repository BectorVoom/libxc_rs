//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 582/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk582<F: Float>(t6921: F, t6934: F, t6948: F, t6917: F, t6929: F, t6938: F, t6941: F, t6946: F, t6953: F, t7181: F) -> (F, F, F, F) {
    let t7183 = 0.28260929265898273597e-2 * t6921;
    let t7185 = 0.67287926823567318088e-4 * t6934;
    let t7189 = 7.0 / 1152.0 * t6948;
    let t7191 = -t7181 - t6917 / 24.0 - t7183 - 0.24223653656484234512e-2 * t6929 - t7185 - 0.40372756094140390853e-3 * t6938 + t6941 / 768.0 - t6946 / 768.0 - t7189 - t6953 / 192.0;
    (t7183, t7185, t7189, t7191)
}
