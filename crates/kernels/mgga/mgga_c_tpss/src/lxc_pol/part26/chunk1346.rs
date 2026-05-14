//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1346/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1346<F: Float>(t5: F, t72865: F, t72892: F, t72919: F, t72949: F, t72997: F, t73026: F, t73052: F, t73081: F, t117: F, t4637: F, t5983: F, t69032: F, t69051: F, t69053: F, t69055: F, t69057: F, t69059: F, t69062: F, t69064: F, t69066: F, t69068: F, t69071: F, t69074: F, t69076: F, t69078: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t73085 = piecewise3(t8, 0.0, t72865 + t72892 + t72919 + t72949 + t72997 + t73026 + t73052 + t73081);
    let t73086 = t73085 * t117;
    let t73089 = t5983 * t4637;
    let t73096 = t69032 + t69051 + t69053 + t69055 + t69057 + t69059 + t69062 + t69064 + t69066 + t69068 + t69071 + t69074 + t69076 + t69078;
    (t73086, t73089, t73096)
}
