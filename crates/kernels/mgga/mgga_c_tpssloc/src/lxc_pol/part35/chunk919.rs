//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 919/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk919<F: Float>(t1495: F, t210: F, t5544: F, t10026: F, t10029: F, t13368: F, t16942: F, t16954: F, t16988: F, t16990: F, t16993: F, t16995: F, t17000: F, t2571: F, t13087: F, t13182: F, t13234: F, t16848: F, t16877: F, t16879: F, t20882: F, t20887: F, t20891: F, t20896: F, t20958: F, t20998: F, t2643: F, t843: F) -> (F,) {
    let t21008 = t210 * t1495 * t5544;
    let t21011 = 7.0 / 1536.0 * t16942 + 7.0 / 384.0 * t16954 - 35.0 / 384.0 * t16988 + 7.0 / 192.0 * t16990 - t10026 - 7.0 / 16.0 * t16993 + 7.0 / 48.0 * t16995 - 7.0 / 1536.0 * t17000 - t10029 - 119.0 / 1152.0 * t13368 + 3.0 / 16.0 * t2571 * t21008;
    let t21013 = -35.0 / 72.0 * t13087 - 119.0 / 4608.0 * t13182 + t2643 * t20882 / 256.0 + t2643 * t20887 / 256.0 - t2643 * t20891 / 1024.0 - 7.0 / 192.0 * t16848 - 5.0 / 128.0 * t843 * t20896 + 119.0 / 4608.0 * t13234 + 7.0 / 768.0 * t16877 - 7.0 / 768.0 * t16879 + t20958 + t20998 + t21011;
    (t21013,)
}
