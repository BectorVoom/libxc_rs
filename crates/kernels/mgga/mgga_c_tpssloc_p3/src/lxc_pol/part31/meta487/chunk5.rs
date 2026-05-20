//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1665/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1665<F: Float>(t5308: F, t9016: F, t15868: F, t2095: F, t5161: F, t7217: F, t113: F, t19456: F, t1983: F, t2040: F, t2096: F, t22574: F, t24987: F, t24995: F, t26161: F, t26559: F, t26870: F, t26872: F, t4028: F, t6876: F, t7050: F, t7057: F, t7171: F, t7220: F, t7685: F, t7904: F, t7943: F) -> (F, F, F, F) {
    let t26875 = t9016 * t5308;
    let t26878 = t2095 * t15868;
    let t26880 = t7217 * t5161;
    let t26895 = -t113 * t26870 - F::new(2.0) * t19456 * t2040 - t1983 * t26878 - t1983 * t26880 + t2096 * t24987 - F::new(3.0) * t22574 * t26872 + F::new(6.0) * t24995 * t26875 + F::new(2.0) * t26161 * t26559 - F::new(2.0) * t4028 * t7050 - F::new(2.0) * t4028 * t7057 + F::new(3.0) * t6876 * t7904 - t6876 * t7943 + F::new(3.0) * t7171 * t7685 - t7220 * t7685;
    (t26875, t26878, t26880, t26895)
}
