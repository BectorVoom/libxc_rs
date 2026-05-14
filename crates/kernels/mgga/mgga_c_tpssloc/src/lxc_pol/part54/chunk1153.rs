//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1153/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1153<F: Float>(t6562: F, t82133: F, t8547: F, t7106: F, t857: F, t225: F, t31362: F, t23030: F, t31405: F, t31315: F, t794: F, t23012: F, t8548: F, t214: F, t7084: F, t31329: F, t6547: F) -> (F, F, F, F, F, F, F, F) {
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114811 = t31362 * t225;
    let t114814 = t23030 * t31405;
    let t114815 = 0.26044789391763585244e-1 * t114814;
    let t114827 = t6562 * t794 * t31315;
    let t114864 = t23012 * t8548;
    let t114865 = 0.63969658155208805863e-1 * t114864;
    let t114866 = t214 * t7084;
    let t114882 = t6547 * t31329;
    (t114795, t114797, t114811, t114815, t114827, t114865, t114866, t114882)
}
