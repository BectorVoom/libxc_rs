//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2018/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2018<F: Float>(t91000: F, t91010: F, t91113: F, t91120: F, t91094: F, t91096: F, t91098: F, t91101: F, t91103: F, t91105: F, t91107: F, t91109: F, t91116: F, t91118: F, t91122: F, t91124: F, t91126: F, t91128: F, t91130: F) -> (F, F, F) {
    let t93615 = F::cast_from(0.12793931631041761173e0_f64) * t91000;
    let t93618 = F::cast_from(0.15352717957250113407e0_f64) * t91010;
    let t93633 = F::new(7.0) / F::new(288.0) * t91113;
    let t93636 = F::new(7.0) / F::new(576.0) * t91120;
    let t93642 = t91094 / F::new(192.0) + t91096 / F::new(192.0) + t91098 / F::new(384.0) + t91101 / F::new(96.0) - F::new(5.0) / F::new(192.0) * t91103 + t91105 / F::new(128.0) - t91107 / F::new(768.0) - t91109 / F::new(384.0) - t93633 + t91116 / F::new(192.0) + t91118 / F::new(192.0) + t93636 + t91122 / F::new(96.0) + t91124 / F::new(96.0) + t91126 / F::new(96.0) + t91128 / F::new(96.0) + t91130 / F::new(192.0);
    (t93615, t93618, t93642)
}
