//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 534/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk534<F: Float>(t1986: F, t2403: F, t675: F, t13862: F, t572: F, t3133: F, t2318: F, t305: F, t13866: F, t2281: F, t2001: F, t3141: F, t552: F, t793: F, t2060: F, t8975: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15220 = t1986 * t2403;
    let t15221 = t675 * t15220;
    let t15223 = t13862 * t572;
    let t15224 = t3133 * t15223;
    let t15226 = t305 * t2318;
    let t15227 = t1986 * t15226;
    let t15228 = t13866 * t15227;
    let t15230 = t305 * t2281;
    let t15231 = t2001 * t15230;
    let t15232 = t3141 * t15231;
    let t15234 = t793 * t552;
    let t15235 = t1986 * t15234;
    let t15236 = t3141 * t15235;
    let t15238 = t2060 * t8975;
    (t15220, t15221, t15223, t15224, t15227, t15228, t15231, t15232, t15235, t15236, t15238)
}
