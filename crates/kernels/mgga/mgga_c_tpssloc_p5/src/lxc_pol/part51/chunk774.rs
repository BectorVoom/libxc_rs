//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 774/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk774<F: Float>(t6899: F, t1323: F, t2085: F, t6914: F, t6921: F, t6934: F, t6948: F, t6917: F, t6929: F, t6938: F, t6941: F, t6946: F, t6953: F) -> (F, F, F, F, F, F, F) {
    let t7176 = F::cast_from(0.82246703342411321825e-2_f64) * t6899;
    let t7179 = t1323 * t2085;
    let t7181 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6914;
    let t7183 = F::cast_from(0.28260929265898273597e-2_f64) * t6921;
    let t7185 = F::cast_from(0.67287926823567318088e-4_f64) * t6934;
    let t7189 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t6948;
    let t7191 = -t7181 - t6917 / F::cast_from(24.0_f64) - t7183 - F::cast_from(0.24223653656484234512e-2_f64) * t6929 - t7185 - F::cast_from(0.40372756094140390853e-3_f64) * t6938 + t6941 / F::cast_from(768.0_f64) - t6946 / F::cast_from(768.0_f64) - t7189 - t6953 / F::cast_from(192.0_f64);
    (t7176, t7179, t7181, t7183, t7185, t7189, t7191)
}
