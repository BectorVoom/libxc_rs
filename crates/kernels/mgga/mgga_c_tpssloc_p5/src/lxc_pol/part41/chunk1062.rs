//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1062/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1062<F: Float>(t5576: F, t838: F, t119: F, t16662: F, t210: F, t4180: F, t4181: F, t4234: F, t16839: F, t829: F, t16891: F, t10014: F, t10026: F, t10029: F, t10036: F, t13359: F, t13362: F, t13368: F, t16985: F, t16988: F, t16990: F, t16993: F, t16995: F, t16997: F, t249: F, t2623: F, t2643: F, t5624: F, t5628: F, t787: F, t843: F) -> F {
    let t17000 = t5576 * t838;
    let t17003 = t119 * t16662;
    let t17004 = t210 * t17003;
    let t17009 = t4180 * t4181 * t4234;
    let t17013 = t4180 * t16839 * t829;
    let t17017 = t4180 * t16891 * t829;
    let t17020 = F::new(5.0) / F::new(768.0) * t2623 * t5624 - t2623 * t5628 / F::new(768.0) - t843 * t16985 / F::new(768.0) - F::new(35.0) / F::new(1152.0) * t16988 + F::new(7.0) / F::new(576.0) * t16990 + F::new(119.0) / F::new(13824.0) * t10014 - t10026 - F::new(7.0) / F::new(48.0) * t16993 + F::new(7.0) / F::new(144.0) * t16995 + t16997 * t249 / F::new(3072.0) - F::new(7.0) / F::new(4608.0) * t17000 - t10029 + t13359 + t13362 - F::new(119.0) / F::new(1728.0) * t13368 - t787 * t17004 / F::new(48.0) - F::new(35.0) / F::new(216.0) * t10036 - t2643 * t17009 / F::new(1536.0) - t2643 * t17013 / F::new(3072.0) - t2643 * t17017 / F::new(3072.0);
    t17020
}
