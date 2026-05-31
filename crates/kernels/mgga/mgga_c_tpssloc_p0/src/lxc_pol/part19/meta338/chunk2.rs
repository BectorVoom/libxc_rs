//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1206/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206<F: Float>(t10047: F, t225: F, t2742: F, t9587: F, t9585: F, t10046: F, t10049: F, t10104: F, t10110: F, t10112: F, t10116: F, t259: F, t2591: F, t2710: F, t2713: F, t2718: F, t2719: F, t2720: F, t2743: F, t798: F, t855: F, t866: F, t9593: F) -> F {
    let t40852 = t10047 * t225;
    let t40866 = t2742 * t2742;
    let t40870 = t9587 * t225;
    let t40875 = t9585 * t225;
    let t40887 = -F::cast_from(36.0_f64) * t10110 * t2719 * t2742 * t855 + F::cast_from(4.0_f64) * t10046 * t259 * t798 + F::cast_from(6.0_f64) * t259 * t2591 * t2710 + F::cast_from(6.0_f64) * t2718 * t40866 * t855 + F::cast_from(12.0_f64) * t10049 * t2720 - F::cast_from(4.0_f64) * t10104 * t2713 - F::cast_from(24.0_f64) * t10112 * t2713 + F::cast_from(24.0_f64) * t10116 * t2713 + F::cast_from(24.0_f64) * t2720 * t9593 - F::cast_from(12.0_f64) * t2743 * t9593 - F::cast_from(4.0_f64) * t40852 * t866 - F::cast_from(12.0_f64) * t40870 * t866 - F::cast_from(4.0_f64) * t40875 * t866;
    t40887
}
