//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1562;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta351<F: Float>(t13222: F, t16968: F, t16673: F, t842: F, t13345: F, t13365: F, t1516: F, t16914: F, t16918: F, t16924: F, t16928: F, t16932: F, t16937: F, t16940: F, t16942: F, t16946: F, t16951: F, t16954: F, t16957: F, t16961: F, t16965: F, t2571: F, t2643: F, t4172: F, t4178: F, t4261: F, t5593: F, t843: F, t849: F, t9559: F, t9642: F, t16662: F, t820: F, t847: F, t2697: F, t5624: F, t13360: F, t5568: F, t9573: F, t2563: F, t5572: F, t16805: F, t237: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16969, t16976, t16979) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1562::<F>(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1563::<F>(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
    (t16969, t16976, t16979, t16985, t16988, t16990, t16993, t16995, t16997)
}
