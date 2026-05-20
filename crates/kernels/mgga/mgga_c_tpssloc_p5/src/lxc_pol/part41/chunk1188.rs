//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1188/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1188<F: Float>(t1118: F, t18834: F, t1099: F, t11185: F, t6024: F, t1128: F, t6031: F, t11211: F, t11317: F, t14702: F, t15072: F, t15074: F, t18742: F, t18747: F, t18749: F, t18752: F, t18755: F, t18757: F) -> (F, F, F, F) {
    let t18835 = t18834 * t1118;
    let t18837 = F::new(1.0) * t1099 * t18835;
    let t18839 = F::cast_from(0.16081979498692535067e2_f64) * t11185 * t6024;
    let t18840 = t6031 * t1128;
    let t18869 = F::new(0.6311625e0) * t18742 - t11317 + F::cast_from(0.45908888888888888888e0_f64) * t14702 - t15072 - t15074 + F::cast_from(0.11577222222222222222e0_f64) * t11211 - F::cast_from(0.157790625e0_f64) * t18747 + F::new(0.6311625e0) * t18749 + F::new(0.31558125e0) * t18752 + F::cast_from(0.264729375e1_f64) * t18755 - F::new(0.3529725e1) * t18757;
    (t18837, t18839, t18840, t18869)
}
