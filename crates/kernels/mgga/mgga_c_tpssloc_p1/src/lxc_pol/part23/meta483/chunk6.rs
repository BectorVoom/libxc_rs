//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1469/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469<F: Float>(t1174: F, t1214: F, t1227: F, t1230: F, t15740: F, t22149: F, t22154: F, t22218: F, t22301: F, t248: F, t3440: F, t3508: F, t45037: F, t4889: F, t5024: F, t52836: F, t66057: F, t72703: F, t72705: F, t72708: F, t72727: F, t72733: F, t72798: F, t77981: F, t78031: F, t79018: F) -> F {
    let t79188 = F::new(7.0) / F::new(1536.0) * t45037 * t248 * t1214 * t79018 * t3508 + t52836 * t22301 / F::new(768.0) + t5024 * t22218 / F::new(36.0) - t1227 * t248 * t1230 * t77981 / F::new(4608.0) + t72703 / F::new(27.0) + F::new(2.0) / F::new(27.0) * t72705 + t72708 / F::new(27.0) - t66057 / F::new(162.0) - t15740 * t22154 / F::new(384.0) - F::new(4.0) / F::new(27.0) * t4889 * t22149 + t1174 * t3440 * t78031 / F::new(54.0) - t72727 / F::new(288.0) - F::new(209.0) / F::new(972.0) * t72733 + F::new(19.0) / F::new(216.0) * t72798;
    t79188
}
