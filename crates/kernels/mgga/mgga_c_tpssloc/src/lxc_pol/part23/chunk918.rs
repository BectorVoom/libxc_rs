//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 918/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk918<F: Float>(t13113: F, t16702: F, t185: F, t20234: F, t9897: F, t1462: F, t16689: F, t13124: F, t16711: F, t20822: F, t9853: F, t9859: F, t9907: F, t9921: F, t20811: F, t20812: F, t20821: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t20823 = 0.17544670867903938621e1 * t13113;
    let t20824 = 12.0 * t16702;
    let t20825 = t185 * t20234;
    let t20827 = 24.0 * t9897 * t20825;
    let t20829 = 12.0 * t16689 * t1462;
    let t20830 = 0.32530743900905219526e-1 * t13124;
    let t20831 = 0.54934341918019635162e-3 * t16711;
    let t20832 = t20822 + t9907 - t20823 + t20824 + t20827 + t9853 + t20829 - t9921 + t20830 - t20831 + t9859;
    let t20835 = (t20811 + t20812 + t20821 + t20832) * t225;
    (t20823, t20824, t20825, t20827, t20829, t20830, t20831, t20835)
}
