//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1062/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1062<F: Float>(t1449: F, t3882: F, t11294: F, t11356: F, t11362: F, t14804: F, t14807: F, t14810: F, t14813: F, t14817: F, t14820: F, t14824: F, t14830: F, t14835: F, t2550: F, t2575: F, t2594: F, t2619: F, t3849: F, t3865: F, t3887: F, t8847: F, t8888: F) -> (F,) {
    let t14838 = t1449 * t3882;
    let t14841 = 0.64327917994770140268e2 * t11294 * t3849 + 6.0 * t2575 * t14804 - 4.0 * t2550 * t14807 - 0.19298375398431042081e3 * t8847 * t14810 - 2.0 * t2550 * t14813 + 0.32163958997385070134e2 * t2575 * t14817 + 0.64327917994770140268e2 * t2575 * t14820 + 0.2069040516770936012e4 * t8888 * t14824 - t14830 - 0.23392894490538584828e1 * t11362 * t3865 + 0.34631718211362927517e2 * t11356 * t3887 + 0.35089341735807877242e1 * t2619 * t14835 - 0.23392894490538584828e1 * t2594 * t14838;
    (t14841,)
}
