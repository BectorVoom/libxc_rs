//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1100/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1100<F: Float>(t1042: F, t5117: F, t9292: F, t5114: F, t2862: F, t2913: F, t5113: F, t2911: F, t4104: F, t4108: F, t5081: F, t9495: F, t9493: F, t4192: F, t4198: F, t4181: F, t4197: F) -> (F, F, F, F, F, F, F) {
    let t15616 = t5117 * t1042;
    let t15618 = 0.96491876992155210402e2 * t9292 * t15616;
    let t15619 = t5114 * t1042;
    let t15621 = 2.0 * t2862 * t15619;
    let t15622 = t5113 * t2913;
    let t15623 = t15622 * t1042;
    let t15625 = 0.16081979498692535067e2 * t2911 * t15623;
    let t15626 = t4108 * t4104;
    let t15628 = 0.32163958997385070134e2 * t2911 * t15626;
    let t15629 = t5081 * t9495;
    let t15630 = t15629 * t1042;
    let t15632 = 0.51726012919273400301e3 * t9493 * t15630;
    let t15634 = 0.23392894490538584828e1 * t4192 * t4198;
    let t15635 = t4197 * t4181;
    (t15618, t15621, t15625, t15628, t15632, t15634, t15635)
}
