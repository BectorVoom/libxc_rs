//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1040/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1040<F: Float>(t361: F, t3949: F, t949: F, t3931: F, t2704: F, t3932: F, t2732: F, t11453: F, t3934: F, t2722: F, t11026: F, t3923: F, t10353: F, t929: F, t926: F, t2685: F, t2731: F, t3924: F, t3935: F, t8577: F, t8588: F, t8954: F, t8966: F, t8972: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t11491 = t361 * t3949;
    let t11492 = t11491 * t949;
    let t11493 = t3931 * t11492;
    let t11496 = t3932 * t2704;
    let t11497 = t3931 * t11496;
    let t11500 = t3932 * t2732;
    let t11501 = t3931 * t11500;
    let t11506 = t11453 * t3934;
    let t11508 = t2722 * t11506 / 1152.0;
    let t11509 = t3923 * t11026;
    let t11512 = t929 * t10353;
    let t11513 = t926 * t11512;
    let t11518 = t8588 / 81.0 - t8954 / 10368.0 - t8966 / 432.0 - t2731 * t11493 / 1536.0 - t2731 * t11497 / 3072.0 + t8577 * t11501 / 3072.0 - t8972 * t3935 / 144.0 + t11508 + t925 * t11509 / 48.0 + t925 * t11513 / 288.0 + t2685 * t3924 / 27.0;
    (t11491, t11493, t11497, t11501, t11506, t11513, t11518)
}
