//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 742/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk742<F: Float>(t1341: F, t638: F, t703: F, t7310: F, t69701: F, t69760: F, t69819: F, t69832: F, t69860: F, t69865: F, t14567: F, t942: F) -> (F, F, F, F, F, F, F, F) {
    let t71446 = t638 * t7310 * t703 * t1341;
    let t71447 = F::new(0.30487649791575028314e-3) * t71446;
    let t71448 = F::new(0.22800128353348964998e-6) * t69701;
    let t71486 = F::new(0.10986805899793472145e-3) * t69760;
    let t71502 = F::new(0.19516036795685772888e-4) * t69819;
    let t71505 = F::new(0.68400385060046895e-6) * t69832;
    let t71513 = F::new(0.69390353051327192491e-4) * t69860;
    let t71514 = F::new(0.13010691197123848592e-4) * t69865;
    let t71516 = t942 * t14567;
    (t71447, t71448, t71486, t71502, t71505, t71513, t71514, t71516)
}
